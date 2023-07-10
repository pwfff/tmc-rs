use bitfield_struct::bitfield;
use embedded_hal::blocking::spi::Transfer;

use tmc_rs_macros::Register;

use super::Register;
use super::Registers;

pub const TMC2240_REGISTER_COUNT: usize = 128;

#[derive(Debug, Default)]
pub struct TMC2240 {
    // 0x00 - 0x0F
    pub GCONF: GCONF,
    pub GSTAT: GSTAT,
    pub IFCNT: IFCNT,
    pub NODECONF: NODECONF,
    pub IOIN: IOIN,
    pub DRV_CONF: DRV_CONF,
    pub GLOBAL_SCALER: GLOBAL_SCALER,

    // 0x10 - 0x1F
    pub IHOLD_IRUN: IHOLD_IRUN,
    pub TPOWERDOWN: TPOWERDOWN,
    pub TSTEP: TSTEP,
    pub TPWMTHRS: TPWMTHRS,
    pub TCOOLTHRS: TCOOLTHRS,
    pub THIGH: THIGH,

    // 0x20 - 0x2F
    pub DIRECT_MODE: DIRECT_MODE,

    // 0x30 - 0x3F
    pub ENCMODE: ENCMODE,
    pub XENC: X_ENC,
    pub ENC_CONST: ENC_CONST,
    pub ENC_STATUS: ENC_STATUS,
    pub ENC_LATCH: ENC_LATCH,

    // 0x40 - 0x4F

    // 0x50 - 0x5F
    pub ADC_VSUPPLY_AIN: ADC_VSUPPLY_AIN,
    pub ADC_TEMP: ADC_TEMP,
    pub OTW_OV_VTH: OTW_OV_VTH,

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
    pub DRVSTATUS: DRV_STATUS,

    // 0x70 - 0x7F
    pub PWMCONF: PWMCONF,
    pub PWMSCALE: PWM_SCALE,
    pub PWM_AUTO: PWM_AUTO,
    pub SG4_THRS: SG4_THRS,
    pub SG4_RESULT: SG4_RESULT,
    pub SG4_IND: SG4_IND,
}

impl Registers<TMC2240_REGISTER_COUNT> for TMC2240 {
    fn reset<S: Transfer<u8>>(&self, spi: &mut S) -> Result<(), S::Error> {
        self.GCONF.write(spi)?;
        self.DRV_CONF.write(spi)?;
        self.ENC_CONST.write(spi)?;
        self.OTW_OV_VTH.write(spi)?;
        self.MSLUT0.write(spi)?;
        self.MSLUT1.write(spi)?;
        self.MSLUT2.write(spi)?;
        self.MSLUT3.write(spi)?;
        self.MSLUT4.write(spi)?;
        self.MSLUT5.write(spi)?;
        self.MSLUT6.write(spi)?;
        self.MSLUT7.write(spi)?;
        self.MSLUTSEL.write(spi)?;
        self.MSLUTSTART.write(spi)?;
        self.CHOPCONF.write(spi)?;
        self.PWMCONF.write(spi)?;

        Ok(())
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

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x00)]
pub struct GCONF {
    #[rustfmt::skip] #[skip] __: u8,

    #[rustfmt::skip] #[bits(7)] __: u8,
    pub direct_mode: bool,

    pub stop_enable: bool,
    pub small_hysteris: bool,
    pub diag1_pushpull: bool,
    pub diag0_pushpull: bool,
    #[rustfmt::skip] #[skip] __: bool,
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
    #[rustfmt::skip] #[skip] __: bool,
}

impl Default for GCONF {
    fn default() -> Self {
        Self::new().with_multistep_filt(true)
    }
}

#[cfg(test)]
mod test {
    use crate::registers::{tmc2240::CHOPCONF, Register};

    use super::GCONF;

    macro_rules! hex_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(
                $a,
                $b,
                "{:02x?} != {:02x?}",
                $a,
                $b,
            );
        };
        ($a:expr, $b:expr,) => {
            hex_eq!($a, $b);
        };
    }

    #[test]
    fn default_bytes() {
        let mut foo = GCONF::default();
        hex_eq!(
            foo.bytes(),
            [0, 0, 0, 0x08],
        );
        foo.set_en_pwm_mode(true);
        assert_eq!(foo.bytes(), [0, 0, 0, 0x0C]);

        //Self::new()
        //    .with_intpol(true)
        //    .with_TPFD(0x4)
        //    .with_TBL(0b10)
        //    .with_HENDOFFSET(0x2)
        //    .with_HSTRT_TFD210(0x5)
        let bar = CHOPCONF::default();
        hex_eq!(
            bar.bytes(),
            [0x10, 0x41, 0x01, 0x50],
        );
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x01)]
pub struct GSTAT {
    __: u8,

    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(3)] __: u8,
    pub vm_uvlo: bool,
    pub register_reset: bool,
    pub uv_cp: bool,
    pub drv_err: bool,
    pub reset: bool,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x02)]
pub struct IFCNT {
    __: u8,

    __: u8,

    __: u8,

    pub interface_transmission_counter: u8,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
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

impl SENDDELAY {
    // This has to be a const fn
    const fn into_bits(self) -> u32 {
        self as _
    }
    const fn from_bits(value: u32) -> Self {
        match value {
            0 => Self::EIGHT,
            2 => Self::THREE_X_EIGHT,
            4 => Self::FIVE_X_EIGHT,
            6 => Self::SEVEN_X_EIGHT,
            8 => Self::NINE_X_EIGHT,
            10 => Self::ELEVEN_X_EIGHT,
            12 => Self::THIRTEEN_X_EIGHT,
            14 => Self::FIFTEEN_X_EIGHT,
            _ => Self::EIGHT,
        }
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x03)]
pub struct NODECONF {
    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(4)]
    pub SENDDELAY: SENDDELAY,

    pub NODEADDR: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x04)]
pub struct IOIN {
    pub VERSION: u8,

    #[rustfmt::skip] #[bits(5)] __: u8,
    #[bits(3)]
    pub SILICON_RV: u8,

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

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOPE_CONTROL {
    V_PER_US_100,
    V_PER_US_200,
    V_PER_US_400,
    V_PER_US_800,
}

impl SLOPE_CONTROL {
    // This has to be a const fn
    const fn into_bits(self) -> u32 {
        self as _
    }
    const fn from_bits(value: u32) -> Self {
        match value {
            0 => Self::V_PER_US_100,
            1 => Self::V_PER_US_200,
            2 => Self::V_PER_US_400,
            3 => Self::V_PER_US_800,
            _ => Self::V_PER_US_100,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURRENT_RANGE {
    ONE_AMP,
    TWO_AMP,
    THREE_AMP,
    ALSO_THREE_AMP,
}

impl CURRENT_RANGE {
    // This has to be a const fn
    const fn into_bits(self) -> u32 {
        self as _
    }
    const fn from_bits(value: u32) -> Self {
        match value {
            0 => Self::ONE_AMP,
            1 => Self::TWO_AMP,
            2 => Self::THREE_AMP,
            3 => Self::ALSO_THREE_AMP,
            _ => Self::ONE_AMP,
        }
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x0A)]
pub struct DRV_CONF {
    __: u8,

    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(2)] __: u8,
    #[bits(2)]
    pub SLOPE_CONTROL: SLOPE_CONTROL,
    #[rustfmt::skip] #[bits(2)] __: u8,
    #[bits(2)]
    pub CURRENT_RANGE: CURRENT_RANGE,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x0B)]
pub struct GLOBAL_SCALER {
    __: u8,

    __: u8,

    __: u8,

    // note that 0 is full current, and 1-31 are not allowed for operation
    pub GLOBALSCALER: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x10)]
pub struct IHOLD_IRUN {
    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(4)]
    pub IRUNDELAY: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(4)]
    pub IHOLDDELAY: u8,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(5)]
    pub IRUN: u8,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(5)]
    pub IHOLD: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x11)]
pub struct TPOWERDOWN {
    __: u8,

    __: u8,

    __: u8,

    pub TPOWERDOWN: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x12)]
pub struct TSTEP {
    __: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(20)]
    pub TSTEP: u32,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x13)]
pub struct TPWMTHRS {
    __: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(20)]
    pub TPWMTHRS: u32,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x14)]
pub struct TCOOLTHRS {
    __: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(20)]
    pub TCOOLTHRS: u32,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x15)]
pub struct THIGH {
    __: u8,

    #[rustfmt::skip] #[bits(4)] __: u8,
    #[bits(20)]
    pub THIGH: u32,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x2D)]
pub struct DIRECT_MODE {
    #[rustfmt::skip] #[bits(7)] __: u8,
    pub DIRECT_COIL_A_NEGATIVE: bool,

    pub DIRECT_COIL_A: u8,

    #[rustfmt::skip] #[bits(7)] __: u8,
    pub DIRECT_COIL_B_NEGATIVE: bool,

    pub DIRECT_COIL_B: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x38)]
pub struct ENCMODE {
    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(5)] __: u8,
    pub enc_sel_decimal: bool,
    #[rustfmt::skip] #[skip] __: bool,
    pub clr_enc_x: bool,

    #[bits(2)]
    pub pos_neg_edge: u8,
    pub clr_once: bool,
    pub clr_cont: bool,
    pub ignore_AB: bool,
    pub pol_N: bool,
    pub pol_B: bool,
    pub pol_A: bool,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x39)]
pub struct X_ENC {
    pub X_ENC: i32,
}

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x3A)]
pub struct ENC_CONST {
    pub ENC_CONST: i32,
}

impl Default for ENC_CONST {
    fn default() -> Self {
        Self::new().with_ENC_CONST(0x00010000)
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x3B)]
pub struct ENC_STATUS {
    __: u8,

    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(7)] __: u8,
    pub n_event: bool,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x3C)]
pub struct ENC_LATCH {
    pub ENC_LATCH: u32,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x50)]
pub struct ADC_VSUPPLY_AIN {
    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    pub ADC_AIN: u16,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    pub ADC_VSUPPLY: u16,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x51)]
pub struct ADC_TEMP {
    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    _RESERVED: u16,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    pub ADC_TEMP: u16,
}

impl ADC_TEMP {
    pub fn degrees_c(&self) -> f32 {
        ((self.ADC_TEMP() - 2038) as f32) / 7.7
    }
}

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x52)]
pub struct OTW_OV_VTH {
    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    pub OVERTEMPPREWARNING_VTH: u16,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(13)]
    pub OVERVOLTAGE_VTH: u16,
}

impl Default for OTW_OV_VTH {
    fn default() -> Self {
        Self::new()
            .with_OVERTEMPPREWARNING_VTH(0xB92)
            .with_OVERVOLTAGE_VTH(0xF25)
    }
}

// TODO: helpers for all this mslut stuff...
macro_rules! mslut {
    ($name:ident, $addr:literal, $default:literal) => {
        #[bitfield(u32, default = false)]
        #[derive(Register, Eq, PartialEq)]
        #[addr($addr)]
        pub struct $name {
            pub $name: u32,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::from($default)
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

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x68)]
pub struct MSLUTSEL {
    pub X3: u8,

    pub X2: u8,

    pub X1: u8,

    #[bits(2)]
    pub W3: u8,
    #[bits(2)]
    pub W2: u8,
    #[bits(2)]
    pub W1: u8,
    #[bits(2)]
    pub W0: u8,
}

impl Default for MSLUTSEL {
    fn default() -> Self {
        Self::from(0xFFFF8056_u32)
    }
}

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x69)]
pub struct MSLUTSTART {
    pub OFFSET_SIN90: u8,

    pub START_SIN90: u8,

    __: u8,

    pub START_SIN: u8,
}

impl Default for MSLUTSTART {
    fn default() -> Self {
        Self::from(0x00F70000_u32)
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x6A)]
pub struct MSCNT {
    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(7)] __: u8,
    #[bits(9)]
    pub MSCNT: u16,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x6B)]
pub struct MSCURACT {
    #[rustfmt::skip] #[bits(7)] __: u8,
    #[bits(9)]
    pub CUR_A: u16,

    #[rustfmt::skip] #[bits(7)] __: u8,
    #[bits(9)]
    pub CUR_B: u16,
}

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x6C)]
pub struct CHOPCONF {
    pub diss2vs: bool,
    pub diss2g: bool,
    pub dedge: bool,
    pub intpol: bool,
    #[bits(4)]
    pub MRES: u8,

    #[bits(4)]
    pub TPFD: u8,
    pub vhighchm: bool,
    pub vhighfs: bool,
    #[rustfmt::skip] #[skip] __: bool,
    #[bits(2)]
    pub TBL: u8,

    pub chm: bool,
    #[rustfmt::skip] #[skip] __: bool,
    pub disfdcc: bool,
    pub fd3: bool,
    #[bits(4)]
    pub HENDOFFSET: u8,

    #[bits(3)]
    pub HSTRT_TFD210: u8,
    #[bits(4)]
    pub TOFF: u8,
}

impl Default for CHOPCONF {
    fn default() -> Self {
        Self::new()
            .with_intpol(true)
            .with_TPFD(0x4)
            .with_TBL(0b10)
            .with_HENDOFFSET(0x2)
            .with_HSTRT_TFD210(0x5)
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x6D)]
pub struct COOLCONF {
    #[rustfmt::skip] #[bits(7)] __: u8,
    pub sfilt: bool,

    #[rustfmt::skip] #[skip] __: bool,
    #[bits(7)]
    pub sgt: u8,

    pub seimin: bool,
    #[bits(2)]
    pub sedn: u8,
    #[rustfmt::skip] #[skip] __: bool,
    #[bits(4)]
    pub semax: u8,

    #[rustfmt::skip] #[skip] __: bool,
    #[bits(2)]
    pub seup: u8,
    #[rustfmt::skip] #[skip] __: bool,
    #[bits(4)]
    pub semin: u8,
}

#[bitfield(u8)]
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

impl SPI_STATUS {
    // This has to be a const fn
    const fn into_bits(self) -> u32 {
        self.0 as u32
    }
    const fn from_bits(value: u32) -> Self {
        let mut new = Self::new();
        new.0 = value as u8;
        new
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x6F)]
pub struct DRV_STATUS {
    #[bits(8)]
    pub SPI_STATUS: SPI_STATUS,

    #[rustfmt::skip] #[bits(3)] __: u8,
    #[bits(5)]
    pub CS_ACTUAL: u8,

    pub fsactive: bool,
    pub stealth: bool,
    pub s2vsb: bool,
    pub s2vsa: bool,
    #[rustfmt::skip] #[bits(2)] __: u8,
    #[bits(10)]
    pub SG_RESULT: u16,
}

#[bitfield(u32, default = false)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x70)]
pub struct PWMCONF {
    #[bits(4)]
    pub PWM_LIM: u8,
    #[bits(4)]
    pub PWM_REG: u8,

    pub pwm_dis_reg_stst: bool,
    pub pwm_meas_sd_enable: bool,
    #[bits(2)]
    pub FREEWHEEL: u8,
    pub pwm_autograd: bool,
    pub pwm_autoscale: bool,
    #[bits(2)]
    pub PWM_FREQ: u8,

    pub PWM_GRAD: u8,

    pub PWM_OFS: u8,
}

impl Default for PWMCONF {
    fn default() -> Self {
        Self::new()
            .with_PWM_LIM(0xC)
            .with_PWM_REG(0x4)
            .with_pwm_autograd(true)
            .with_pwm_autoscale(true)
            .with_PWM_OFS(0x1D)
    }
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x71)]
pub struct PWM_SCALE {
    #[rustfmt::skip] #[bits(7)] __: u8,
    #[bits(9)]
    pub PWM_SCALE_AUTO: u16,

    #[rustfmt::skip] #[bits(7)] __: u8,
    #[bits(9)]
    pub PWM_SCALE_SUM: u16,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x72)]
pub struct PWM_AUTO {
    __: u8,

    pub PWM_GRAD_AUTO: u8,

    __: u8,

    pub PWM_OFS_AUTO: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x74)]
pub struct SG4_THRS {
    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(6)] __: u8,
    pub sg_angle_offset: bool,
    pub sg4_filt_en: bool,

    pub SG4_THRS: u8,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x75)]
pub struct SG4_RESULT {
    __: u8,

    __: u8,

    #[rustfmt::skip] #[bits(6)] __: u8,
    #[bits(10)]
    pub SG4_RESULT: u16,
}

#[bitfield(u32)]
#[derive(Register, Eq, PartialEq)]
#[addr(0x76)]
pub struct SG4_IND {
    pub SG4_IND_3: u8,

    pub SG4_IND_2: u8,

    pub SG4_IND_1: u8,

    pub SG4_IND_0: u8,
}
