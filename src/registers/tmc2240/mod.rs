use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;
use modular_bitfield::BitfieldSpecifier;
use tmc_rs_macros::Register;

use super::Register;
use super::Registers;
use super::WritableRegister;

pub const TMC2240_REGISTER_COUNT: usize = 128;

#[bitfield]
pub struct TMC2240 {
    // 0x00 - 0x0F
    pub GCONF: GCONF,
    pub GSTAT: GSTAT,
    pub IFCNT: IFCNT,
    pub NODECONF: NODECONF,
    pub IOIN: IOIN,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    pub DRV_CONF: DRV_CONF,
    pub GLOBAL_SCALER: GLOBAL_SCALER,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x10 - 0x1F
    pub IHOLD_IRUN: IHOLD_IRUN,
    pub TPOWERDOWN: TPOWERDOWN,
    pub TSTEP: TSTEP,
    pub TPWMTHRS: TPWMTHRS,
    pub TCOOLTHRS: TCOOLTHRS,
    pub THIGH: THIGH,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x20 - 0x2F
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    pub DIRECT_MODE: DIRECT_MODE,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x30 - 0x3F
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    pub ENCMODE: ENCMODE,
    pub XENC: X_ENC,
    pub ENC_CONST: ENC_CONST,
    pub ENC_STATUS: ENC_STATUS,
    pub ENC_LATCH: ENC_LATCH,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x40 - 0x4F
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x50 - 0x5F
    pub ADC_VSUPPLY_AIN: ADC_VSUPPLY_AIN,
    pub ADC_TEMP: ADC_TEMP,
    pub OTW_OV_VTH: OTW_OV_VTH,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,

    // 0x60 - 0x6F
    pub MSLUT0: MSLUT_0,
    pub MSLUT1: MSLUT_1,
    pub MSLUT2: MSLUT_2,
    pub MSLUT3: MSLUT_3,
    pub MSLUT4: MSLUT_4,
    pub MSLUT5: MSLUT_5,
    pub MSLUT6: MSLUT_6,
    pub MSLUT7: MSLUT_7,
    pub MSLUTSEL: MSLUTSEL,
    pub MSLUTSTART: MSLUTSTART,
    pub MSCNT: MSCNT,
    pub MSCURACT: MSCURACT,
    pub CHOPCONF: CHOPCONF,
    pub COOLCONF: COOLCONF,
    #[rustfmt::skip] #[skip] __: B32,
    pub DRVSTATUS: DRV_STATUS,

    // 0x70 - 0x7F
    pub PWMCONF: PWMCONF,
    pub PWMSCALE: PWM_SCALE,
    pub PWM_AUTO: PWM_AUTO,
    #[rustfmt::skip] #[skip] __: B32,
    pub SG4_THRS: SG4_THRS,
    pub SG4_RESULT: SG4_RESULT,
    pub SG4_IND: SG4_IND,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
    #[rustfmt::skip] #[skip] __: B32,
}

impl Default for TMC2240 {
    fn default() -> Self {
        Self::new()
            .with_GCONF(GCONF::default())
            .with_DRV_CONF(DRV_CONF::default())
            .with_ENC_CONST(ENC_CONST::default())
            .with_OTW_OV_VTH(OTW_OV_VTH::default())
            .with_MSLUT0(MSLUT_0::default())
            .with_MSLUT1(MSLUT_1::default())
            .with_MSLUT2(MSLUT_2::default())
            .with_MSLUT3(MSLUT_3::default())
            .with_MSLUT4(MSLUT_4::default())
            .with_MSLUT5(MSLUT_5::default())
            .with_MSLUT6(MSLUT_6::default())
            .with_MSLUT7(MSLUT_7::default())
            .with_MSLUTSEL(MSLUTSEL::default())
            .with_MSLUTSTART(MSLUTSTART::default())
            .with_CHOPCONF(CHOPCONF::default())
            .with_PWMCONF(PWMCONF::default())
    }
}

impl Registers<TMC2240_REGISTER_COUNT> for TMC2240 {
    fn reset(&self, write_spi: &mut impl FnMut(u8, u32)) {
        self.GCONF().write(write_spi);
        self.DRV_CONF().write(write_spi);
        self.ENC_CONST().write(write_spi);
        self.OTW_OV_VTH().write(write_spi);
        self.MSLUT0().write(write_spi);
        self.MSLUT1().write(write_spi);
        self.MSLUT2().write(write_spi);
        self.MSLUT3().write(write_spi);
        self.MSLUT4().write(write_spi);
        self.MSLUT5().write(write_spi);
        self.MSLUT6().write(write_spi);
        self.MSLUT7().write(write_spi);
        self.MSLUTSEL().write(write_spi);
        self.MSLUTSTART().write(write_spi);
        self.CHOPCONF().write(write_spi);
        self.PWMCONF().write(write_spi);
    }
}

//#[rustfmt::skip]
//const TMC2240_REGISTER_RESET_STATE: [u32; TMC2240_REGISTER_COUNT] = [
// 0xX0,0xX1,0xX2,0xX3,0xX4,0xX5,0xX6,0xX7,0xX8,0xX9,0xXA,0xXB,0xXC,0xXD,0xXE,0xXF
//  R00, ___, ___, ___, ___, ___, ___, ___, ___, ___, R0A, ___, ___, ___, ___, ___, // 0x00 - 0x0F
//  ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0x10 - 0x1F
//  ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0x20 - 0x2F
//  ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, R3A, ___, ___, ___, ___, ___, // 0x30 - 0x3F
//  ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0x40 - 0x4F
//  ___, ___, R52, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0x50 - 0x5F
//  R60, R61, R62, R63, R64, R65, R66, R67, R68, R69, ___, ___, R6C, ___, ___, ___, // 0x60 - 0x6F
//  R70, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0x70 - 0x7F
//];

//impl Registers<TMC2240_REGISTER_COUNT> for TMC2240 {
//    fn get_reset_state() -> [u32; TMC2240_REGISTER_COUNT] {
//        Self::new().reset()
//    }
//}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x00)]
pub struct GCONF {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B7,
    pub direct_mode: bool,

    pub stop_enable: bool,
    pub small_hysteris: bool,
    pub diag1_pushpull: bool,
    pub diag0_pushpull: bool,
    #[rustfmt::skip] #[skip] __: B1,
    pub diag1_onstate: bool,
    pub diag1_index: bool,
    pub diag1_stall: bool,

    pub diag0_stall: bool,
    pub diag0_otpw: bool,
    pub diag0_error: bool,
    pub shaft: bool,
    pub multistep_filt: bool,
    pub en_pwm_mode: bool,
    pub fast_standstill: bool,
    #[rustfmt::skip] #[skip] __: B1,
}

impl WritableRegister for GCONF {
    fn default() -> Self {
        Self::new().with_multistep_filt(true)
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x01)]
pub struct GSTAT {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B3,
    pub vm_uvlo: bool,
    pub register_reset: bool,
    pub uv_cp: bool,
    pub drv_err: bool,
    pub reset: bool,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x02)]
pub struct IFCNT {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    pub interface_transmission_counter: u8,
}

#[derive(BitfieldSpecifier, Debug)]
pub enum SENDDELAY {
    EIGHT,
    _1,
    THREE_X_EIGHT,
    _3,
    FIVE_X_EIGHT,
    _5,
    SEVEN_X_EIGHT,
    _7,
    NINE_X_EIGHT,
    _9,
    ELEVEN_X_EIGHT,
    _11,
    THIRTEEN_X_EIGHT,
    _13,
    FIFTEEN_X_EIGHT,
    _15,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x03)]
pub struct NODECONF {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B4,
    pub SENDDELAY: SENDDELAY,

    pub NODEADDR: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x04)]
pub struct IOIN {
    pub VERSION: u8,

    #[rustfmt::skip] #[skip] __: B5,
    pub SILICON_RV: B3,

    pub ADC_ERR: bool,
    pub EXT_CLK: bool,
    pub EXT_RES_DET: bool,
    pub OUTPUT: bool,
    pub COMP_B1_B2: bool,
    pub COMP_A1_A2: bool,
    pub COMP_B: bool,
    pub COMP_A: bool,

    #[rustfmt::skip] #[skip] _reserved: bool,
    pub UART_EN: bool,
    pub ENCN: bool,
    pub DRV_ENN: bool,
    pub ENCA: bool,
    pub ENCB: bool,
    pub DIR: bool,
    pub STEP: bool,
}

#[derive(BitfieldSpecifier, Debug)]
pub enum SLOPE_CONTROL {
    V_PER_US_100,
    V_PER_US_200,
    V_PER_US_400,
    V_PER_US_800,
}

#[derive(BitfieldSpecifier, Debug)]
pub enum CURRENT_RANGE {
    ONE_AMP,
    TWO_AMP,
    THREE_AMP,
    ALSO_THREE_AMP,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x0A)]
pub struct DRV_CONF {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B2,
    pub SLOPE_CONTROL: SLOPE_CONTROL,
    #[rustfmt::skip] #[skip] __: B2,
    pub CURRENT_RANGE: CURRENT_RANGE,
}

impl WritableRegister for DRV_CONF {
    fn default() -> Self {
        Self::new()
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x0B)]
pub struct GLOBAL_SCALER {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    // note that 0 is full current, and 1-31 are not allowed for operation
    pub GLOBALSCALER: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x10)]
pub struct IHOLD_IRUN {
    #[rustfmt::skip] #[skip] __: B4,
    pub IRUNDELAY: B4,

    #[rustfmt::skip] #[skip] __: B4,
    pub IHOLDDELAY: B4,

    #[rustfmt::skip] #[skip] __: B3,
    pub IRUN: B5,

    #[rustfmt::skip] #[skip] __: B3,
    pub IHOLD: B5,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x11)]
pub struct TPOWERDOWN {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    pub TPOWERDOWN: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x12)]
pub struct TSTEP {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B4,
    pub TSTEP: B20,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x13)]
pub struct TPWMTHRS {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B4,
    pub TPWMTHRS: B20,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x14)]
pub struct TCOOLTHRS {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B4,
    pub TCOOLTHRS: B20,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x15)]
pub struct THIGH {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B4,
    pub THIGH: B20,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x2D)]
pub struct DIRECT_MODE {
    #[rustfmt::skip] #[skip] __: B7,
    pub DIRECT_COIL_A_NEGATIVE: bool,

    pub DIRECT_COIL_A: u8,

    #[rustfmt::skip] #[skip] __: B7,
    pub DIRECT_COIL_B_NEGATIVE: bool,

    pub DIRECT_COIL_B: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x38)]
pub struct ENCMODE {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B5,
    pub enc_sel_decimal: bool,
    #[rustfmt::skip] #[skip] __: B1,
    pub clr_enc_x: bool,

    pub pos_neg_edge: B2,
    pub clr_once: bool,
    pub clr_cont: bool,
    pub ignore_AB: bool,
    pub pol_N: bool,
    pub pol_B: bool,
    pub pol_A: bool,
}

// TODO: move
#[bitfield]
#[derive(BitfieldSpecifier, Debug)]
pub struct SignedInteger {
    pub sign: bool,
    pub value: B31,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x39)]
pub struct X_ENC {
    pub X_ENC: SignedInteger,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x3A)]
pub struct ENC_CONST {
    pub ENC_CONST: SignedInteger,
}

impl WritableRegister for ENC_CONST {
    fn default() -> Self {
        Self::new().with_ENC_CONST(SignedInteger::new().with_value(0x00010000))
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x3B)]
pub struct ENC_STATUS {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B7,
    pub n_event: bool,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x3C)]
pub struct ENC_LATCH {
    pub ENC_LATCH: u32,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x50)]
pub struct ADC_VSUPPLY_AIN {
    #[rustfmt::skip] #[skip] __: B3,
    pub ADC_AIN: B13,

    #[rustfmt::skip] #[skip] __: B3,
    pub ADC_VSUPPLY: B13,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x51)]
pub struct ADC_TEMP {
    #[rustfmt::skip] #[skip] __: B3,
    _RESERVED: B13,

    #[rustfmt::skip] #[skip] __: B3,
    pub ADC_TEMP: B13,
}

impl ADC_TEMP {
    pub fn degrees_c(&self) -> f32 {
        ((self.ADC_TEMP() - 2038) as f32) / 7.7
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x52)]
pub struct OTW_OV_VTH {
    #[rustfmt::skip] #[skip] __: B3,
    pub OVERTEMPPREWARNING_VTH: B13,

    #[rustfmt::skip] #[skip] __: B3,
    pub OVERVOLTAGE_VTH: B13,
}

impl WritableRegister for OTW_OV_VTH {
    fn default() -> Self {
        Self::new()
            .with_OVERTEMPPREWARNING_VTH(0xB92)
            .with_OVERVOLTAGE_VTH(0xF25)
    }
}

// TODO: helpers for all this mslut stuff...
macro_rules! mslut {
    ($name:ident, $addr:literal, $default:literal) => {
        #[bitfield]
        #[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
        #[addr($addr)]
        pub struct $name {
            pub $name: B32,
        }

        impl WritableRegister for $name {
            fn default() -> Self {
                Self::from_bytes($default.to_be_bytes())
            }
        }
    };
}

mslut!(MSLUT_0, 0x60, 0xAAAAB554_u32);
mslut!(MSLUT_1, 0x61, 0x4A9554AA_u32);
mslut!(MSLUT_2, 0x62, 0x24492929_u32);
mslut!(MSLUT_3, 0x63, 0x10104222_u32);
mslut!(MSLUT_4, 0x64, 0xFBFFFFFF_u32);
mslut!(MSLUT_5, 0x65, 0xB5BB777D_u32);
mslut!(MSLUT_6, 0x66, 0x49295556_u32);
mslut!(MSLUT_7, 0x67, 0x00404222_u32);

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x68)]
pub struct MSLUTSEL {
    pub X3: u8,

    pub X2: u8,

    pub X1: u8,

    pub W3: B2,
    pub W2: B2,
    pub W1: B2,
    pub W0: B2,
}

impl WritableRegister for MSLUTSEL {
    fn default() -> Self {
        Self::from_bytes(0xFFFF8056_u32.to_be_bytes())
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x69)]
pub struct MSLUTSTART {
    pub OFFSET_SIN90: u8,

    pub START_SIN90: u8,

    #[rustfmt::skip] #[skip] __: B8,

    pub START_SIN: u8,
}

impl WritableRegister for MSLUTSTART {
    fn default() -> Self {
        Self::from_bytes(0x00F70000_u32.to_be_bytes())
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x6A)]
pub struct MSCNT {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B7,
    pub MSCNT: B9,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x6B)]
pub struct MSCURACT {
    #[rustfmt::skip] #[skip] __: B7,
    pub CUR_A: B9,

    #[rustfmt::skip] #[skip] __: B7,
    pub CUR_B: B9,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x6C)]
pub struct CHOPCONF {
    pub diss2vs: bool,
    pub diss2g: bool,
    pub dedge: bool,
    pub intpol: bool,
    pub MRES: B4,

    pub TPFD: B4,
    pub vhighchm: bool,
    pub vhighfs: bool,
    #[rustfmt::skip] #[skip] __: B1,
    pub TBL: B2,

    pub chm: bool,
    #[rustfmt::skip] #[skip] __: B1,
    pub disfdcc: bool,
    pub fd3: bool,
    pub HENDOFFSET: B4,

    pub HSTRT_TFD210: B3,
    pub TOFF: B4,
}

impl WritableRegister for CHOPCONF {
    fn default() -> Self {
        Self::new()
            .with_intpol(true)
            .with_TPFD(0x4)
            .with_TBL(0b10)
            .with_HENDOFFSET(0x2)
            .with_HSTRT_TFD210(0x5)
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x6D)]
pub struct COOLCONF {
    #[rustfmt::skip] #[skip] __: B7,
    pub sfilt: bool,

    #[rustfmt::skip] #[skip] __: B1,
    pub sgt: B7,

    pub seimin: bool,
    pub sedn: B2,
    #[rustfmt::skip] #[skip] __: B1,
    pub semax: B4,

    #[rustfmt::skip] #[skip] __: B1,
    pub seup: B2,
    #[rustfmt::skip] #[skip] __: B1,
    pub semin: B4,
}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
pub struct SPI_STATUS {
    pub stst: bool,
    pub olb: bool,
    pub ola: bool,
    pub s2gb: bool,
    pub s2ga: bool,
    pub otpw: bool,
    pub ot: bool,
    pub stallguard: bool,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x6F)]
pub struct DRV_STATUS {
    pub SPI_STATUS: SPI_STATUS,

    #[rustfmt::skip] #[skip] __: B3,
    pub CS_ACTUAL: B5,

    pub fsactive: bool,
    pub stealth: bool,
    pub s2vsb: bool,
    pub s2vsa: bool,
    #[rustfmt::skip] #[skip] __: B2,
    pub SG_RESULT: B10,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x70)]
pub struct PWMCONF {
    pub PWM_LIM: B4,
    pub PWM_REG: B4,

    pub pwm_dis_reg_stst: bool,
    pub pwm_meas_sd_enable: bool,
    pub FREEWHEEL: B2,
    pub pwm_autograd: bool,
    pub pwm_autoscale: bool,
    pub PWM_FREQ: B2,

    pub PWM_GRAD: u8,

    pub PWM_OFS: u8,
}

impl WritableRegister for PWMCONF {
    fn default() -> Self {
        Self::new()
            .with_PWM_LIM(0xC)
            .with_PWM_REG(0x4)
            .with_pwm_autograd(true)
            .with_pwm_autoscale(true)
            .with_PWM_OFS(0x1D)
    }
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x71)]
pub struct PWM_SCALE {
    #[rustfmt::skip] #[skip] __: B7,
    pub PWM_SCALE_AUTO: B9,

    #[rustfmt::skip] #[skip] __: B7,
    pub PWM_SCALE_SUM: B9,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x72)]
pub struct PWM_AUTO {
    #[rustfmt::skip] #[skip] __: B8,

    pub PWM_GRAD_AUTO: u8,

    #[rustfmt::skip] #[skip] __: B8,

    pub PWM_OFS_AUTO: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x74)]
pub struct SG4_THRS {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B6,
    pub sg_angle_offset: bool,
    pub sg4_filt_en: bool,

    pub SG4_THRS: u8,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x75)]
pub struct SG4_RESULT {
    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B8,

    #[rustfmt::skip] #[skip] __: B6,
    pub SG4_RESULT: B10,
}

#[bitfield]
#[derive(Register, BitfieldSpecifier, Copy, Clone, Eq, PartialEq, Debug)]
#[addr(0x76)]
pub struct SG4_IND {
    pub SG4_IND_3: u8,

    pub SG4_IND_2: u8,

    pub SG4_IND_1: u8,

    pub SG4_IND_0: u8,
}
