#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;

#[cfg(test)]
mod tests {
    use super::bindings::*;

    #[test]
    fn it_works() {
        let motor = TMC2240TypeDef{
            config: todo!(),
            velocity: todo!(),
            oldX: todo!(),
            oldTick: todo!(),
            registerResetState: todo!(),
            registerAccess: todo!(),
            slaveAddress: todo!(),
        };
        unsafe {
            tmc2240_init(motor, channel, config, registerResetState);
        }
        assert_eq!(4, 4);
    }
}
