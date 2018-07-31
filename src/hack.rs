// XXX: Ugly hack that's useful when developing, will never be public.
extern crate core;

use hal::blocking::spi::{Transfer, Write};
use hal::digital::OutputPin;
use lowlevel;
use Cc1101;

impl<SPI, CS, E> core::ops::Deref for Cc1101<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    type Target = lowlevel::Cc1101<SPI, CS>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<SPI, CS, E> core::ops::DerefMut for Cc1101<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    fn deref_mut(&mut self) -> &mut lowlevel::Cc1101<SPI, CS> {
        &mut self.0
    }
}
