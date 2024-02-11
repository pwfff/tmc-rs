use embedded_hal::spi::SpiDevice;
use log::*;

pub mod tmc2240;

pub trait Registers<const REG_COUNT: usize> {
    fn reset<S: SpiDevice>(&self, spi: &mut S) -> Result<(), S::Error>;
}

pub trait Register: From<u32> + Into<u32> + Default {
    fn addr() -> u8;
    fn bytes(self) -> [u8; 4];

    fn read<S: SpiDevice>(&self, spi: &mut S) -> Result<Self, S::Error> {
        let mut write = [Self::addr() & !0x80, 0, 0, 0, 0];
        let mut read = [Self::addr() & !0x80, 0, 0, 0, 0];
        spi.transfer(&mut read, &mut write)?;
        spi.transfer(&mut read, &mut write)?;
        Ok(Self::from(u32::from_be_bytes([
            read[1], read[2], read[3], read[4],
        ])))
    }

    fn write<S: SpiDevice>(self, spi: &mut S) -> Result<(), S::Error> {
        let bytes = self.bytes();
        let addr = Self::addr();

        info!("writing {:02x?} to {:02x}", bytes, addr);

        let mut write = [addr | 0x80, bytes[0], bytes[1], bytes[2], bytes[3]];
        let mut read = [0,0,0,0,0];
        spi.transfer(&mut read, &mut write)?;

        Ok(())
    }
}
