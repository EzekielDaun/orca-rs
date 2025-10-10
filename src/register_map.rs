use bondrewd::BitfieldEnum;
use embedded_registers::register;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(
    BitfieldEnum,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Default,
    defmt::Format,
    Deserialize,
    Serialize,
    IntoPrimitive,
    TryFromPrimitive,
    Copy,
)]
#[bondrewd_enum(u8)]
pub enum OrcaModeOfOperation {
    #[default]
    SleepMode = 1,
    ForceMode = 2,
    PositionMode = 3,
    HapticMode = 4,
    KineticMode = 5,
    PulseWidthMode = 11,
    AutoZeroingMode = 55,
}

#[repr(u8)]
#[derive(
    BitfieldEnum,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Default,
    defmt::Format,
    Deserialize,
    Serialize,
    IntoPrimitive,
    TryFromPrimitive,
    Copy,
)]
#[bondrewd_enum(u8)]
pub enum OrcaZeroMode {
    #[default]
    NegativeZeroing = 0,
    ManualZeroing = 1,
    AutoZeroEnabled = 2,
    AutoZeroOnBoot = 3,
    IOSHAutoZeroing = 4,
}

#[repr(u8)]
#[derive(
    BitfieldEnum,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Default,
    defmt::Format,
    Deserialize,
    Serialize,
    IntoPrimitive,
    TryFromPrimitive,
    Copy,
)]
#[bondrewd_enum(u8)]
pub enum OrcaAutoZeroExitMode {
    #[default]
    SleepMode = 1,
    ForceMode = 2,
    PositionMode = 3,
    HapticMode = 4,
    KineticMode = 5,
    PulseWidthMode = 11,
}

#[register(address = 0x0, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CtrlReg0 {
    reset: bool,
    clear_errors: bool,
    zero_position: bool,
    invert_position: bool,
    #[bondrewd(bit_length = 12, reserve)]
    reserve: u16,
}

#[register(address = 0x1, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CtrlReg1 {
    #[bondrewd(bit_length = 10, reserve)]
    reserve: u16,
    position_controller_gain_set_flag: bool,
    current_controller_gain_set_flag: bool,
    #[bondrewd(bit_length = 4, reserve)]
    reserve2: u8,
}

#[register(address = 0x2, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CtrlReg2 {
    #[bondrewd(bit_length = 4, reserve)]
    reserve: u8,
    tuning_save: bool,
    user_opt_save: bool,
    motion_config_save: bool,
    iosh_save: bool,
    haptic_config_save: bool,
    #[bondrewd(bit_length = 7, reserve)]
    reserve2: u8,
}

#[register(address = 0x3, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CtrlReg3 {
    #[bondrewd(enum_primitive = "u8")]
    mode: OrcaModeOfOperation,
    #[bondrewd(bit_length = 8, reserve)]
    reserve: u8,
}

#[register(address = 0x4, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CtrlReg4 {
    #[bondrewd(bit_length = 1, reserve)]
    reserve: bool,
    tuning_defaults: bool,
    motor_user_options_defaults: bool,
    modbus_user_options_defaults: bool,
    kinematic_defaults: bool,
    haptic_defaults: bool,
    iosh_defaults: bool,
    pwm_defaults: bool,
    #[bondrewd(bit_length = 8, reserve)]
    reserve2: u8,
}

#[register(address = 0x9, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct KinSwTrig {
    #[bondrewd(bit_length = 4)]
    motion_id: u8,
    #[bondrewd(bit_length = 12, reserve)]
    reserve: u16,
}

#[register(address = 28, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ForceCmdL {
    force_cmd_l: u16,
}
#[register(address = 29, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ForceCmdH {
    force_cmd_h: u16,
}

#[register(address = 30, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PosCmdL {
    pos_cmd_l: u16,
}
#[register(address = 31, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PosCmdH {
    pos_cmd_h: u16,
}

#[register(address = 129, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CCPGain {
    cc_pgain: u16,
}

#[register(address = 130, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CCIGain {
    cc_igain: u16,
}

#[register(address = 131, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CCFGain {
    cc_fgain: u16,
}

#[register(address = 132, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CCMaxDuty {
    cc_max_duty: u16,
}

#[register(address = 133, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCPGain {
    pc_pgain: u16,
}

#[register(address = 134, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCIGain {
    pc_igain: u16,
}

#[register(address = 135, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCDVGain {
    pc_dvgain: u16,
}

#[register(address = 136, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCDEGain {
    pc_degain: u16,
}

#[register(address = 137, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCFSatuL {
    pc_fsatu_l: u16,
}

#[register(address = 138, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCFSatuH {
    pc_fsatu_h: u16,
}

#[register(address = 139, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserMaxTemp {
    user_max_temp: u16,
}

#[register(address = 140, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserMaxForceL {
    user_max_force_l: u16,
}

#[register(address = 141, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserMaxForceH {
    user_max_force_h: u16,
}

#[register(address = 142, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserMaxPower {
    user_max_power: u16,
}

#[register(address = 143, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct SafetyDGain {
    safety_dgain: u16,
}

#[register(address = 147, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserMaxCoilTemp {
    user_max_coil_temp: u16,
}

#[register(address = 148, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct TempErrHysteresis {
    temp_err_hysteresis: u16,
}

#[register(address = 150, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PCSoftstartPeriod {
    pc_softstart_period: u16,
}

#[register(address = 152, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PosSign {
    pos_sign: u16,
}

#[register(address = 162, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct LogPeriod {
    log_period: u16,
}

#[register(address = 163, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UserCommsTimeout {
    user_comms_timeout: u16,
}

#[register(address = 164, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UsrMbBaudLo {
    usr_mb_baud_lo: u16,
}

#[register(address = 165, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UsrMbBaudHi {
    usr_mb_baud_hi: u16,
}

#[register(address = 166, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ForceFilt {
    force_filt: u16,
}

#[register(address = 167, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PosFilt {
    pos_filt: u16,
}

#[register(address = 168, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UsrMbDelay {
    usr_mb_delay: u16,
}

#[register(address = 169, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct UsrMbAddr {
    usr_mb_addr: u16,
}

#[register(address = 171, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ZeroMode {
    #[bondrewd(enum_primitive = "u8")]
    zero_mode: OrcaZeroMode,
    #[bondrewd(bit_length = 8, reserve)]
    reserve: u8,
}

#[register(address = 172, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct AutoZeroForceN {
    auto_zero_force_n: u16,
}

#[register(address = 173, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct AutoZeroExitMode {
    #[bondrewd(enum_primitive = "u8")]
    auto_zero_exit_mode: OrcaAutoZeroExitMode,
    #[bondrewd(bit_length = 8, reserve)]
    reserve: u8,
}
#[register(address = 174, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct MBRS485Mode {
    mb_rs485_mode: u16,
}

#[register(address = 175, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct MbForceFilter {
    mb_force_filter: u16,
}

#[register(address = 176, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct MbPosFilter {
    mb_pos_filter: u16,
}

#[register(address = 177, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct AutoZeroSpeedMmps {
    auto_zero_speed_mmps: u16,
}

#[register(address = 178, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmTimeoutMs {
    pwm_timeout_ms: u16,
}

#[register(address = 179, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmTimeConstMs {
    pwm_time_const_ms: u16,
}

#[register(address = 180, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmMinPosL {
    pwm_min_pos_l: u16,
}

#[register(address = 181, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmMinPosH {
    pwm_min_pos_h: u16,
}

#[register(address = 182, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmMaxPosL {
    pwm_max_pos_l: u16,
}

#[register(address = 183, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmMaxPosH {
    pwm_max_pos_h: u16,
}

#[register(address = 184, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct PwmServoType {
    pwm_servo_type: u16,
}

#[register(address = 317, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ModeOfOperation {
    #[bondrewd(enum_primitive = "u8")]
    mode_of_operation: OrcaModeOfOperation,
    #[bondrewd(bit_length = 8, reserve)]
    reserve: u8,
}

#[register(address = 318, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CalibrationStatus {
    calibration_status: u16,
}

#[register(address = 319, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct KinematicStatus {
    kinematic_status: u16,
}

#[register(address = 336, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct BoardTemp {
    board_temp: u16,
}

#[register(address = 338, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct VddFinal {
    vdd_final: u16,
}

#[register(address = 342, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftPosUmL {
    shaft_pos_um_l: u16,
}

#[register(address = 343, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftPosUmH {
    shaft_pos_um_h: u16,
}

#[register(address = 344, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftSpeedMmpsL {
    shaft_speed_mmps_l: u16,
}

#[register(address = 345, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftSpeedMmpsH {
    shaft_speed_mmps_h: u16,
}

#[register(address = 346, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftAccelMmpssL {
    shaft_accel_mmpss_l: u16,
}

#[register(address = 347, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ShaftAccelMmpssH {
    shaft_accel_mmpss_h: u16,
}

#[register(address = 348, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ForceL {
    force_l: u16,
}

#[register(address = 349, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct ForceH {
    force_h: u16,
}

#[register(address = 350, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct Power {
    power: u16,
}

#[register(address = 351, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct HbaCurrent {
    hba_current: u16,
}

#[register(address = 352, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct HbbCurrent {
    hbb_current: u16,
}

#[register(address = 353, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct HbcCurrent {
    hbc_current: u16,
}

#[register(address = 354, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct HbdCurrent {
    hbd_current: u16,
}

#[register(address = 355, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct AvgPower {
    avg_power: u16,
}

#[register(address = 356, mode = "rw")]
#[bondrewd(default_endianness = "le", read_from = "lsb0", enforce_bytes = 2)]
pub struct CoilTemp {
    coil_temp: u16,
}
