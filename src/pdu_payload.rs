use crate::register_map::OrcaModeOfOperation;
use binrw::{BinRead, BinWrite, binrw, io::Cursor};
use bondrewd::Bitfields;
use num_enum::{IntoPrimitive, TryFromPrimitive};

fn check_adu_crc(data: &[u8]) -> bool {
    let crc = crc::Crc::<u16>::new(&crc::CRC_16_MODBUS);
    let checksum = crc.checksum(&data[..data.len() - 2]);
    let received_crc = u16::from_le_bytes([data[data.len() - 2], data[data.len() - 1]]);
    checksum == received_crc
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct OrcaHighSpeedRequestADU {
    pub slave_address: u8,
    pub pdu: OrcaHighSpeedRequestPDU,
    #[brw(little)]
    crc: u16,
}

impl OrcaHighSpeedRequestADU {
    pub fn new(slave_address: u8, pdu: OrcaHighSpeedRequestPDU) -> Self {
        let mut adu = vec![slave_address];
        let mut pdu_bytes = vec![];
        pdu.write(&mut Cursor::new(&mut pdu_bytes)).unwrap();
        adu.extend_from_slice(&pdu_bytes);
        let crc = crc::Crc::<u16>::new(&crc::CRC_16_MODBUS);
        let checksum = crc.checksum(&adu);
        Self {
            slave_address,
            pdu,
            crc: checksum,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut cur = Cursor::new(Vec::new());
        self.write(&mut cur).expect("binrw write failed");
        cur.into_inner()
    }

    pub fn num_response_bytes(&self) -> usize {
        1 + match self.pdu {
            OrcaHighSpeedRequestPDU::Manage(_) => 9,
            OrcaHighSpeedRequestPDU::Command(_) => 16,
            OrcaHighSpeedRequestPDU::Read(_) => 21,
            OrcaHighSpeedRequestPDU::Write(_) => 17,
        } + 2
    }
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct OrcaHighSpeedResponseADU {
    pub slave_address: u8,
    pub pdu: OrcaHighSpeedResponsePDU,
    #[brw(little)]
    crc: u16,
}
impl OrcaHighSpeedResponseADU {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let adu = Self::read(&mut cursor)?;
        if !check_adu_crc(bytes) {
            anyhow::bail!("CRC check failed");
        }
        Ok(adu)
    }
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OrcaHighSpeedRequestPDU {
    #[brw(magic = 0x41u8)]
    Manage(ManageHighSpeedRequestPDUPayload),
    #[brw(magic = 0x64u8)]
    Command(MotorCommandRequestPDUPayload),
    #[brw(magic = 0x68u8)]
    Read(MotorReadRequestPDUPayload),
    #[brw(magic = 0x69u8)]
    Write(MotorWriteRequestPDUPayload),
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OrcaHighSpeedResponsePDU {
    #[brw(magic = 0x41u8)]
    Manage(ManageHighSpeedResponsePDUPayload),
    #[brw(magic = 0x64u8)]
    Command(MotorCommandResponsePDUPayload),
    #[brw(magic = 0x68u8)]
    Read(MotorReadResponsePDUPayload),
    #[brw(magic = 0x69u8)]
    Write(MotorWriteResponsePDUPayload),
}

#[repr(u8)]
#[binrw]
#[brw(repr=u8)]
#[derive(Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Copy, Clone)]
pub enum FunctionCode {
    Manage = 0x41,
    Command = 0x64,
    Read = 0x68,
    Write = 0x69,
}

#[repr(u16)]
#[binrw]
#[brw(repr=u16)]
#[derive(Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Default, Copy, Clone)]
pub enum ManageHighSpeedRequestSubFunctionCode {
    Enable = 0xFF00,
    #[default]
    Disable = 0x0000,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub struct ManageHighSpeedRequestPDUPayload {
    pub sub_function_code: ManageHighSpeedRequestSubFunctionCode,
    pub baud_rate: u32,
    pub delay_us: u16,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ManageHighSpeedResponsePDUPayload {
    pub state_command: ManageHighSpeedRequestSubFunctionCode,
    pub baud_rate: u32,
    pub delay_us: u16,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MotorCommandRequestPDUPayload {
    #[brw(magic = 0x1Cu8)]
    ForceControlStream { force_mn: i32 },

    #[brw(magic = 0x1Eu8)]
    PositionControlStream { position_um: i32 },

    #[brw(magic = 0x20u8)]
    KinematicDataStream {
        #[br(temp)]
        #[brw(calc = 0u32)]
        _pad4: u32,
    },

    #[brw(magic = 0x22u8)]
    HapticDataStream { haptic_status_register: u32 },

    SleepDataStream {
        #[brw(pad_after = 4)]
        #[br(temp)]
        #[brw(calc = 0u8)]
        _unknown_subcode: u8,
    },
}

#[derive(Bitfields, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct OrcaErrors {
    pub configuration_errors: bool,
    #[bondrewd(bit_length = 4, reserve)]
    pub reserve0: u8,
    pub force_clipping: bool,
    pub temperature_exceeded: bool,
    pub force_exceeded: bool,
    pub power_exceeded: bool,
    pub shaft_image_failed: bool,
    pub voltage_invalid: bool,
    pub communication_timeout: bool,
    #[bondrewd(bit_length = 1, reserve)]
    pub reserve1: u8,
    pub auto_zero_failed: bool,
    #[bondrewd(bit_length = 2, reserve)]
    pub reserve2: u16,
}

impl From<u16> for OrcaErrors {
    fn from(value: u16) -> Self {
        Self::from_bytes(value.to_be_bytes())
    }
}
impl From<OrcaErrors> for u16 {
    fn from(val: OrcaErrors) -> Self {
        u16::from_be_bytes(val.into_bytes())
    }
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MotorCommandResponsePDUPayload {
    pub position_um: i32,
    pub force_mn: i32,
    pub power_w: u16,
    pub temperature_c: u8,
    pub voltage_mv: u16,
    #[br(map=|x: u16| OrcaErrors::from(x))]
    #[bw(map=|x: &OrcaErrors| <u16 as From<OrcaErrors>>::from(*x))]
    pub error: OrcaErrors,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MotorReadRequestPDUPayload {
    pub register_address: u16,
    pub register_width: u8,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MotorReadResponsePDUPayload {
    pub read_register_value: u32,
    #[br(map=|x: u8| OrcaModeOfOperation::try_from(x).unwrap_or_default())]
    #[bw(map=|x: &OrcaModeOfOperation| <u8 as From<OrcaModeOfOperation>>::from(*x))]
    pub mode_of_operation: OrcaModeOfOperation,
    pub command_response: MotorCommandResponsePDUPayload,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MotorWriteRequestPDUPayload {
    pub register_address: u16,
    pub register_width: u8,
    pub register_data: u32,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MotorWriteResponsePDUPayload {
    #[br(map=|x: u8| OrcaModeOfOperation::try_from(x).unwrap_or_default())]
    #[bw(map=|x: &OrcaModeOfOperation| <u8 as From<OrcaModeOfOperation>>::from(*x))]
    pub mode_of_operation: OrcaModeOfOperation,
    pub command_response: MotorCommandResponsePDUPayload,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::BinRead;
    use binrw::io::Cursor;

    #[test]
    fn manage_high_speed_stream_frame() {
        let mut bytes_request = Cursor::new(vec![0xFF, 0x00, 0x00, 0x09, 0x89, 0x68, 0x00, 0x32]);
        bytes_request.set_position(0);
        let deserialized_request =
            ManageHighSpeedRequestPDUPayload::read(&mut Cursor::new(bytes_request.into_inner()))
                .unwrap();
        let expected_request = ManageHighSpeedRequestPDUPayload {
            sub_function_code: ManageHighSpeedRequestSubFunctionCode::Enable,
            baud_rate: 625000,
            delay_us: 50,
        };
        assert_eq!(deserialized_request, expected_request);

        let mut bytes_response = Cursor::new(vec![0xFF, 0x00, 0x00, 0x09, 0x89, 0x68, 0x00, 0x32]);
        bytes_response.set_position(0);
        let deserialized_response =
            ManageHighSpeedResponsePDUPayload::read(&mut Cursor::new(bytes_response.into_inner()))
                .unwrap();
        let expected_response = ManageHighSpeedResponsePDUPayload {
            state_command: ManageHighSpeedRequestSubFunctionCode::Enable,
            baud_rate: 625000,
            delay_us: 50,
        };
        assert_eq!(deserialized_response, expected_response);
    }

    #[test]
    fn sleep_stream_command_frame() {
        let mut bytes_request = Cursor::new(vec![0x00, 0x00, 0x00, 0x00, 0x00]);
        bytes_request.set_position(0);
        let deserialized_request =
            MotorCommandRequestPDUPayload::read(&mut Cursor::new(bytes_request.into_inner()))
                .unwrap();
        let expected_request = MotorCommandRequestPDUPayload::SleepDataStream {};
        assert_eq!(deserialized_request, expected_request);
    }

    #[test]
    fn force_control_stream_command_frame() {
        let mut bytes_request = Cursor::new(vec![0x1C, 0x00, 0x00, 0x03, 0xE8]);
        bytes_request.set_position(0);
        let deserialized_request =
            MotorCommandRequestPDUPayload::read(&mut Cursor::new(bytes_request.into_inner()))
                .unwrap();
        let expected_request = MotorCommandRequestPDUPayload::ForceControlStream { force_mn: 1000 };
        assert_eq!(deserialized_request, expected_request);

        let mut bytes_response = Cursor::new(vec![
            0x00, 0x00, 0x2E, 0xE0, 0x00, 0x01, 0x38, 0x80, 0x00, 0x19, 0x18, 0x5E, 0x56, 0x00,
            0x00,
        ]);
        bytes_response.set_position(0);
        let deserialized_response =
            MotorCommandResponsePDUPayload::read(&mut Cursor::new(bytes_response.into_inner()))
                .unwrap();
        let expected_response = MotorCommandResponsePDUPayload {
            position_um: 12000,
            force_mn: 80000,
            power_w: 25,
            temperature_c: 24,
            voltage_mv: 24150,
            error: OrcaErrors::default(),
        };
        assert_eq!(deserialized_response, expected_response);
    }
}
