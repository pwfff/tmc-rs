use modular_bitfield::{prelude::*, error::InvalidBitPattern};

pub mod tmc2240;

pub trait Registers<const REG_COUNT: usize> {
    fn reset(&self, write_spi: &mut impl FnMut(u8, u32));
}

pub trait Register: Specifier<Bytes=<[();32] as ::modular_bitfield::private::SpecifierBytes>::Bytes, InOut = Self> + Sized {
    fn addr() -> u8;
    fn as_word(&self) -> u32;

    fn read(&self, read_word: &mut impl FnMut(u8) -> u32) -> Result<Self, InvalidBitPattern<u32>>
    {
        let got = read_word(<Self as Register>::addr());
        Self::from_bytes(got)
    }
}

pub trait WritableRegister: Register
{
    fn default() -> Self;

    fn get_reset_value() -> (u8, u32) {
        (Self::addr(), Self::default().as_word())
    }

    fn write(&self, write_word: &mut impl FnMut(u8, u32)) {
        write_word(<Self as Register>::addr(), self.as_word())
    }
}
