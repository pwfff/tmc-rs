use embedded_hal::blocking::spi::Transfer;
use modular_bitfield::{error::InvalidBitPattern, prelude::*};
use log::*;

pub mod tmc2240;

pub trait Registers<const REG_COUNT: usize> {
    fn reset<S: Transfer<u8>>(&self, spi: &mut S) -> Result<(), S::Error>;
}

pub trait Register:
    Specifier<
        Bytes = <[(); 32] as ::modular_bitfield::private::SpecifierBytes>::Bytes,
        InOut = Self,
    > + Sized
{
    fn addr() -> u8;
    fn as_word(&self) -> u32;
    fn bytes(&self) -> [u8; 4];

    fn read<S: Transfer<u8>>(&self, spi: &mut S) -> Result<Self, InvalidBitPattern<u32>> {
        let mut write = [Self::addr(), 0, 0, 0, 0];
        spi
            .transfer(&mut write)
            .map_err(|_| InvalidBitPattern::new(123))?;
        let got = spi
            .transfer(&mut write)
            .map_err(|_| InvalidBitPattern::new(123))?;
        Self::from_bytes(u32::from_be_bytes([got[1], got[2], got[3], got[4]]))
    }
}

pub trait WritableRegister: Register {
    fn default() -> Self;

    fn get_reset_value() -> (u8, u32) {
        (Self::addr(), Self::default().as_word())
    }

    fn write<S: Transfer<u8>>(&self, spi: &mut S) -> Result<(), S::Error> {
        let bytes = self.bytes();
        let addr = Self::addr();

        info!("writing {:02x?} to {:02x}", bytes, addr);

        let mut write = [addr | 0x80, bytes[0], bytes[1], bytes[2], bytes[3]];
        spi.transfer(&mut write)?;

        Ok(())
    }
}
