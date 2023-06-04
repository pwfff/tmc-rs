#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub use bindings::*;

mod bindings;

// TODO: how do we make this async safe...
static mut SPIWriter: Option<fn(address: u8, value: i32)> = None;

#[no_mangle]
pub extern "C" fn tmc2240_writeInt(_tmc2240: *mut TMC2240TypeDef, address: u8, value: i32) {
    // from the eval board example... we'll only do SPI tho
    /*
    if(commMode == TMC_BOARD_COMM_SPI)
    {
        spi_writeInt(TMC2240_SPIChannel,  address,  value);
    }
    else if (commMode == TMC_BOARD_COMM_UART)
    {
        //UART_writeInt(TMC2240_UARTChannel,  targetAddressUart,  address,  value);
        tmc2240_UARTwriteInt(TMC2240_UARTChannel,  address,  value);
    }
    */

    unsafe {
        if let Some(writer) = SPIWriter {
            writer(address, value)
        }
    }
}

pub fn new(
    channel: u8,
    mut config: ConfigurationTypeDef,
    registerResetState: &[i32; 128usize],
    callback: tmc2240_callback,
) -> TMC2240TypeDef {
    let mut tmc2240 = TMC2240TypeDef {
        config: core::ptr::null_mut(),
        velocity: 0,
        oldX: 0,
        oldTick: 0,
        registerResetState: [0i32; 128usize],
        registerAccess: [0u8; 128usize],
        slaveAddress: 0,
    };

    unsafe {
        tmc2240_init(
            &mut tmc2240,
            channel,
            &mut config,
            registerResetState.as_ptr(),
        );
        tmc2240_setCallback(&mut tmc2240, callback);
    }

    tmc2240
}

pub fn periodic(tmc2240: &mut TMC2240TypeDef, tick: u32) {
    unsafe { tmc2240_periodicJob(tmc2240, tick) }
}

#[cfg(test)]
mod tests {
    use crate::{new, periodic, SPIWriter};

    use super::bindings::*;

    static mut wrote: bool = false;
    fn fakeSpi(a: u8, v: i32) {
        unsafe {
            wrote = true;
        }
    }

    #[test]
    fn it_works() {
        let config = ConfigurationTypeDef {
            state: ConfigState_CONFIG_RESTORE,
            configIndex: 0,
            shadowRegister: [0i32; 128usize],
            reset: None,
            restore: None,
            callback: None,
            channel: 0,
        };
        let registerResetState = [0i32; 128usize];

        unsafe {
            SPIWriter = Some(fakeSpi);
        }

        let mut ic = new(0, config, &registerResetState, None);
        unsafe {
            tmc2240_reset(&mut ic);
        }

        let mut i = 0;
        unsafe {
            while ((*ic.config).state != ConfigState_CONFIG_READY) && i < 512 {
                i += 1;
                periodic(&mut ic, 12345);
            }
        }
        assert_eq!(4, 4);
        assert_ne!(i, 512);
        assert_ne!(i, 1);
        unsafe {
            assert_eq!(wrote, true);
        }
    }
}
