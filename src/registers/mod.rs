use embedded_hal::blocking::spi::Transfer;
use log::*;

pub mod tmc2240;

pub trait Registers<const REG_COUNT: usize> {
    fn reset<S: Transfer<u8>>(&self, spi: &mut S) -> Result<(), S::Error>;
}

pub trait Register: From<u32> + Into<u32> + Default {
    fn addr() -> u8;
    fn bytes(self) -> [u8; 4];

    fn read<S: Transfer<u8>>(&self, spi: &mut S) -> Result<Self, S::Error> {
        let mut write = [Self::addr() & !0x80, 0, 0, 0, 0];
        spi.transfer(&mut write)?;
        let got = spi.transfer(&mut write)?;
        Ok(Self::from(u32::from_be_bytes([
            got[1], got[2], got[3], got[4],
        ])))
    }

    fn write<S: Transfer<u8>>(self, spi: &mut S) -> Result<(), S::Error> {
        let bytes = self.bytes();
        let addr = Self::addr();

        info!("writing {:02x?} to {:02x}", bytes, addr);

        let mut write = [addr | 0x80, bytes[0], bytes[1], bytes[2], bytes[3]];
        spi.transfer(&mut write)?;

        Ok(())
    }
}
