//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]
//#![no_std]

//pub use bindings::*;

//pub mod bindings;
pub mod registers;

// TODO: how do we make this async safe...
//pub static mut SPIWriter: Option<fn(address: u8, value: i32)> = None;

//#[no_mangle]
//pub extern "C" fn tmc2240_writeInt(_tmc2240: *mut TMC2240TypeDef, address: u8, value: i32) {
//    // from the eval board example... we'll only do SPI tho
//    /*
//    if(commMode == TMC_BOARD_COMM_SPI)
//    {
//        spi_writeInt(TMC2240_SPIChannel,  address,  value);
//    }
//    else if (commMode == TMC_BOARD_COMM_UART)
//    {
//        //UART_writeInt(TMC2240_UARTChannel,  targetAddressUart,  address,  value);
//        tmc2240_UARTwriteInt(TMC2240_UARTChannel,  address,  value);
//    }
//    */
//
//    unsafe {
//        if let Some(writer) = SPIWriter {
//            writer(address, value)
//        }
//    }
//}
//
//// https://github.com/rust-lang/rust-bindgen/issues/1266
//#[rustfmt::skip]
//pub const tmc2240_defaultRegisterResetState: [u32; TMC2240_REGISTER_COUNT as usize] = [
////	0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   A,   B,   C,   D,   E,   F
//    R00, 0,   0,   0,   0,   0,   0,   0,   0,   0,   R0A, 0,   0,   0,   0,   0, // 0x00 - 0x0F
//    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, // 0x10 - 0x1F
//    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, // 0x20 - 0x2F
//    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   R3A, 0,   0,   0,   0,   0, // 0x30 - 0x3F
//    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, // 0x40 - 0x4F
//    0,   0,   R52, 0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, // 0x50 - 0x5F
//    R60, R61, R62, R63, R64, R65, R66, R67, R68, R69, 0,   0,   R6C, 0,   0,   0, // 0x60 - 0x6F
//    R70, 0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, // 0x70 - 0x7F
//];
//
//// Register access permissions:
////   0x00: none (reserved)
////   0x01: read
////   0x02: write
////   0x03: read/write
////   0x13: read/write, separate functions/values for reading or writing
////   0x23: read/write, flag register (write to clear)
////   0x42: write, has hardware presets on reset
//#[rustfmt::skip]
//pub const tmc2240_defaultRegisterAccess: [u8; TMC2240_REGISTER_COUNT as usize] = [
////	0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
//	0x03, 0x23, 0x01, 0x03, 0x03,    0,    0,    0,    0,    0, 0x03, 0x03,    0,    0,    0,    0, // 0x00 - 0x0F
//	0x03, 0x03, 0x01, 0x03, 0x03, 0x03,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 0x10 - 0x1F
//	   0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0, 0x03,    0,    0, // 0x20 - 0x2F
//	   0,    0,    0,    0,    0,    0,    0,    0, 0x03, 0x03, 0x03, 0x23, 0x01,    0,    0,    0, // 0x30 - 0x3F
//	   0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 0x40 - 0x4F
//	0x01, 0x01, 0x03,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 0x50 - 0x5F
//	0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x01, 0x01, 0x03, 0x03,    0, 0x01, // 0x60 - 0x6F
//	0x03, 0x01, 0x01,    0, 0x03, 0x01, 0x01,    0,    0,    0,    0,    0,    0,    0,    0,    0  // 0x70 - 0x7F
//];
//
//pub fn new(
//    channel: u8,
//    mut config: ConfigurationTypeDef,
//    registerResetState: &[i32; 128usize],
//    callback: tmc2240_callback,
//) -> TMC2240TypeDef {
//    let mut tmc2240 = TMC2240TypeDef {
//        config: core::ptr::null_mut(),
//        velocity: 0,
//        oldX: 0,
//        oldTick: 0,
//        registerResetState: [0i32; 128usize],
//        registerAccess: [0u8; 128usize],
//        slaveAddress: 0,
//    };
//
//    unsafe {
//        tmc2240_init(
//            &mut tmc2240,
//            channel,
//            &mut config,
//            registerResetState.as_ptr(),
//        );
//        tmc2240_setCallback(&mut tmc2240, callback);
//    }
//
//    tmc2240
//}
//
//pub fn periodic(tmc2240: &mut TMC2240TypeDef, tick: u32) {
//    unsafe { tmc2240_periodicJob(tmc2240, tick) }
//}
//
//#[derive(Default, Debug)]
//pub struct GStat {
//    pub vm_uvlo: bool,
//    pub register_reset: bool,
//    pub uv_cp: bool,
//    pub drv_err: bool,
//    pub reset: bool,
//}
//
//impl From<u8> for GStat {
//    fn from(value: u8) -> Self {
//        let mut status = Self::default();
//
//        for (f, m) in [
//            (&mut status.reset, TMC2240_RESET_MASK),
//            (&mut status.drv_err, TMC2240_DRV_ERR_MASK),
//            (&mut status.uv_cp, TMC2240_UV_CP_MASK),
//            (&mut status.register_reset, TMC2240_REGISTER_RESET_MASK),
//            (&mut status.vm_uvlo, TMC2240_VM_UVLO_MASK),
//        ] {
//            *f = ((value) & m as u8) > 0;
//        }
//
//        status
//    }
//}
//
//#[derive(Default, Debug)]
//pub struct SPIStatus {
//    pub reset: bool,
//    pub driver_error: bool,
//    pub sg2: bool,
//    pub standstill: bool,
//}
//
//impl From<u8> for SPIStatus {
//    fn from(value: u8) -> Self {
//        let mut status = Self::default();
//
//        for (f, m) in [
//            (&mut status.reset, TMC2240_SPI_STATUS_RESET_FLAG_MASK),
//            (
//                &mut status.driver_error,
//                TMC2240_SPI_STATUS_DRIVER_ERROR_MASK,
//            ),
//            (&mut status.sg2, TMC2240_SPI_STATUS_SG2_MASK),
//            (&mut status.standstill, TMC2240_SPI_STATUS_STANDSTILL_MASK),
//        ] {
//            *f = ((value) & m as u8) > 0;
//        }
//
//        status
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use crate::{new, periodic, SPIWriter};
//
//    use super::bindings::*;
//
//    static mut wrote: bool = false;
//    fn fakeSpi(a: u8, v: i32) {
//        unsafe {
//            wrote = true;
//        }
//    }
//
//    #[test]
//    fn it_works() {
//        let config = ConfigurationTypeDef {
//            state: ConfigState_CONFIG_RESTORE,
//            configIndex: 0,
//            shadowRegister: [0i32; 128usize],
//            reset: None,
//            restore: None,
//            callback: None,
//            channel: 0,
//        };
//        let registerResetState = [0i32; 128usize];
//
//        unsafe {
//            SPIWriter = Some(fakeSpi);
//        }
//
//        let mut ic = new(0, config, &registerResetState, None);
//        unsafe {
//            tmc2240_reset(&mut ic);
//        }
//
//        let mut i = 0;
//        unsafe {
//            while ((*ic.config).state != ConfigState_CONFIG_READY) && i < 512 {
//                i += 1;
//                periodic(&mut ic, 12345);
//            }
//        }
//        assert_eq!(4, 4);
//        assert_ne!(i, 512);
//        assert_ne!(i, 1);
//        unsafe {
//            assert_eq!(wrote, true);
//        }
//    }
//}
