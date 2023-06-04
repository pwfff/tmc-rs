#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub use bindings::*;

mod bindings;

#[no_mangle]
pub extern "C" fn tmc2240_writeInt(tmc2240: *mut TMC2240TypeDef, address: u8, value: i32) {
    // this is where we'll do SPI stuff... somehow...
    unsafe {
        // to see if it worked...
        (*tmc2240).velocity = 999;
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
    use crate::{new, periodic};

    use super::bindings::*;

    unsafe extern "C" fn callback(tmc2240: *mut TMC2240TypeDef, state: ConfigState) {}

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

        let mut ic = new(0, config, &registerResetState, Some(callback));
        unsafe {
            tmc2240_reset(&mut ic);
        }

        let mut i = 0;
        unsafe {
            while ((*ic.config).state != ConfigState_CONFIG_READY) && i < 512 {
                i+=1;
                periodic(&mut ic, 12345);
            }
        }
        assert_eq!(4, 4);
        assert_ne!(i, 512);
        assert_ne!(i, 1);
        assert_eq!(ic.velocity, 999);
    }
}
