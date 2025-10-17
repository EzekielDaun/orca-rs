use futures::future::try_join_all;
use orca_rs::OrcaMotor;
use orca_rs::pdu_payload::OrcaHighSpeedResponsePDU;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Serial ports for multiple motors (adjust to your devices)
    let mut motors: Vec<OrcaMotor<_>> = [
        "/dev/tty.usbserial-FT878BFG",
        "/dev/tty.usbserial-FT8ESWCO",
        "/dev/tty.usbserial-FT8F0YKB",
        "/dev/tty.usbserial-FTU7JPNF",
        "/dev/tty.usbserial-FTU7JOOC",
        "/dev/tty.usbserial-FT8EQ4MF",
        "/dev/tty.usbserial-FT8F11PO",
        "/dev/tty.usbserial-FT8F0SER",
        "/dev/tty.usbserial-FT878A04",
    ]
    .iter()
    .map(|tty| {
        let builder = tokio_serial::new(*tty, 115200)
            .parity(tokio_serial::Parity::Even)
            .timeout(std::time::Duration::from_millis(1));
        let port = tokio_serial::SerialStream::open(&builder)?;
        let port = embedded_io_adapters::tokio_1::FromTokio::new(port);
        Ok(OrcaMotor::new(port))
    })
    .collect::<Result<_, anyhow::Error>>()?;

    // Switch to sleep mode and read it
    try_join_all(
        motors
            .iter_mut()
            .map(|m| m.set_mode(orca_rs::register_map::OrcaModeOfOperation::SleepMode)),
    )
    .await?;
    let modes = try_join_all(motors.iter_mut().map(|m| m.read_mode())).await?;
    for (i, mode) in modes.iter().enumerate() {
        println!("[M{}] Current mode: {:?}", i + 1, mode);
    }

    // Enable the high-speed stream with the same baudrate
    let responses = try_join_all(motors.iter_mut().map(|m| m.enable_high_speed(115200, 5))).await?;
    for (i, resp) in responses.into_iter().enumerate() {
        match resp {
            OrcaHighSpeedResponsePDU::Manage(payload) => {
                println!("[M{}]   Baud rate: {}", i + 1, payload.baud_rate);
                println!("[M{}]   Delay (us): {}", i + 1, payload.delay_us);
            }
            _ => println!("[M{}] Unexpected response PDU", i + 1),
        }
    }

    // Stress-test max communication rate: no delays; each step sends to all motors concurrently and waits for all to complete
    let total_um: i32 = 20_000;
    let steps: i32 = 1_000;
    let step_inc_um: i32 = total_um / steps;

    let mut position_um: i32 = 0;
    let start = std::time::Instant::now();
    for _ in 0..steps {
        position_um += step_inc_um;
        try_join_all(
            motors
                .iter_mut()
                .map(|m| m.send_position_high_speed(position_um)),
        )
        .await?;
    }

    // Statistics (identical per motor; aggregate throughput is N×)
    let elapsed = start.elapsed();
    let elapsed_s = elapsed.as_secs_f64();
    let n = motors.len() as f64;
    let steps_f = steps as f64;
    let cmds_per_sec_per_motor = steps_f / elapsed_s;
    let combined_cmds_per_sec = (steps_f * n) / elapsed_s;
    let avg_us_per_cmd_per_motor = (elapsed_s * 1e6) / steps_f;
    let total_mm = (position_um as f64) / 1000.0;
    let mm_per_sec = total_mm / elapsed_s;
    println!(
        "[SYNC x{}] Max rate: {} steps in {:.3} s -> {:.1} cmds/s per motor ({:.1} combined), avg {:.1} µs/cmd per motor. Distance {:.3} mm, speed {:.3} mm/s",
        n as usize,
        steps,
        elapsed_s,
        cmds_per_sec_per_motor,
        combined_cmds_per_sec,
        avg_us_per_cmd_per_motor,
        total_mm,
        mm_per_sec
    );

    // Disable high-speed and restore sleep mode
    try_join_all(motors.iter_mut().map(|m| m.disable_high_speed())).await?;
    try_join_all(
        motors
            .iter_mut()
            .map(|m| m.set_mode(orca_rs::register_map::OrcaModeOfOperation::SleepMode)),
    )
    .await?;

    Ok(())
}
