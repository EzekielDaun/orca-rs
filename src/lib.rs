pub mod pdu_payload;
pub mod register_map;
use embedded_registers::Register;
use log;
use rmodbus::{ModbusProto, client::ModbusRequest, guess_response_frame_len};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::pdu_payload::*;
use crate::register_map::*;

pub struct OrcaMotor {
    pub port: tokio_serial::SerialStream,
    pub mreq: ModbusRequest,
}

impl OrcaMotor {
    pub fn new(port: tokio_serial::SerialStream) -> Self {
        Self {
            port,
            mreq: ModbusRequest::new(1, ModbusProto::Rtu),
        }
    }
    pub fn new_with_slave(port: tokio_serial::SerialStream, slave: u8) -> Self {
        Self {
            port,
            mreq: ModbusRequest::new(slave, ModbusProto::Rtu),
        }
    }

    pub async fn read_mode(&mut self) -> anyhow::Result<OrcaModeOfOperation> {
        let mut bytes = vec![];
        self.mreq
            .generate_get_holdings(ModeOfOperation::ADDRESS as u16, 1, &mut bytes)?;
        self.port.write_all(&bytes).await?;

        let mut buf = [0u8; 3];
        self.port.read_exact(&mut buf).await?;

        let mut response = Vec::new();
        response.extend_from_slice(&buf);
        let len = guess_response_frame_len(&buf, ModbusProto::Rtu)?;
        if len > 6 {
            let mut rest = vec![0u8; (len - 3) as usize];
            self.port.read_exact(&mut rest).await?;
            response.extend(rest);
        }
        // check if frame has no Modbus error inside
        self.mreq.parse_ok(&response)?;

        let mut data = Vec::new();
        self.mreq.parse_u16(&response, &mut data)?;

        Ok(OrcaModeOfOperation::try_from(data[0] as u8)?)
    }

    pub async fn set_mode(&mut self, mode: OrcaModeOfOperation) -> anyhow::Result<()> {
        let mut bytes = vec![];
        self.mreq
            .generate_set_holding(CtrlReg3::ADDRESS as u16, mode as u16, &mut bytes)?;
        self.port.write_all(&bytes).await?;

        let mut buf = [0u8; 8];
        self.port.read_exact(&mut buf).await?;

        // check if frame has no Modbus error inside
        self.mreq.parse_ok(&buf)?;

        Ok(())
    }

    pub async fn send_high_speed_adu(
        &mut self,
        adu: &OrcaHighSpeedRequestADU,
    ) -> anyhow::Result<OrcaHighSpeedResponsePDU> {
        let adu_vec = adu.to_vec();
        self.port.write_all(&adu_vec).await?;

        let mut buf = vec![0u8; adu.num_response_bytes()];
        self.port.read_exact(&mut buf).await?;
        log::debug!("Response: {buf:?}");

        let response_adu = OrcaHighSpeedResponseADU::from_bytes(&buf)?;
        log::debug!("Response ADU parsed: {response_adu:?}");

        if adu.slave_address != response_adu.slave_address {
            anyhow::bail!("Slave address mismatch");
        }

        Ok(response_adu.pdu)
    }
    pub async fn enable_high_speed(
        &mut self,
        baud_rate: u32,
        delay_us: u16,
    ) -> anyhow::Result<OrcaHighSpeedResponsePDU> {
        self.send_high_speed_adu(&OrcaHighSpeedRequestADU::new(
            self.mreq.unit_id,
            OrcaHighSpeedRequestPDU::Manage(ManageHighSpeedRequestPDUPayload {
                sub_function_code: ManageHighSpeedRequestSubFunctionCode::Enable,
                baud_rate,
                delay_us,
            }),
        ))
        .await
    }
    pub async fn disable_high_speed(&mut self) -> anyhow::Result<OrcaHighSpeedResponsePDU> {
        self.send_high_speed_adu(&OrcaHighSpeedRequestADU::new(
            self.mreq.unit_id,
            OrcaHighSpeedRequestPDU::Manage(ManageHighSpeedRequestPDUPayload {
                sub_function_code: ManageHighSpeedRequestSubFunctionCode::Disable,
                ..Default::default()
            }),
        ))
        .await
    }

    pub async fn send_position_high_speed(
        &mut self,
        position_um: i32,
    ) -> anyhow::Result<OrcaHighSpeedResponsePDU> {
        self.send_high_speed_adu(&OrcaHighSpeedRequestADU::new(
            self.mreq.unit_id,
            OrcaHighSpeedRequestPDU::Command(
                MotorCommandRequestPDUPayload::PositionControlStream { position_um },
            ),
        ))
        .await
    }
}
