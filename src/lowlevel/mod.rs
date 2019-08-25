//! Low level unrestricted access to the CC1101 radio chip.
use hal::blocking::spi::{Transfer, Write};
use hal::digital::v2::OutputPin;

#[macro_use]
mod macros;
mod access;
mod traits;

pub mod registers;
pub mod types;

use self::registers::*;

pub struct Cc1101<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
    //    gdo0: GDO0,
    //    gdo2: GDO2,
}

impl<SPI, CS, E> Cc1101<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin<Error = E>,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let cc1101 = Cc1101 {
            spi: spi,
            cs: cs,
        };
        Ok(cc1101)
    }

    pub fn read_register<R>(&mut self) -> Result<R, E>
    where
        R: RegisterClass,
    {
        self.cs.set_low()?;
        let mut buffer = [R::REGISTER_CLASS.raddr(), 0u8];
        self.spi.transfer(&mut buffer)?;
        self.cs.set_high()?;
        Ok(From::<u8>::from(buffer[1]))
    }

    pub fn read_fifo(&mut self, addr: &mut u8, len: &mut u8, buf: &mut [u8]) -> Result<(), E> {
        let mut buffer = [Command::FIFO.addr() | 0xC0, 0, 0];

        self.cs.set_low()?;
        self.spi.transfer(&mut buffer)?;
        self.spi.transfer(buf)?;
        self.cs.set_high()?;

        *len = buffer[1];
        *addr = buffer[2];

        Ok(())
    }

    pub fn write_strobe(&mut self, com: Command) -> Result<(), E> {
        self.cs.set_low()?;
        self.spi.write(&[com.addr()])?;
        self.cs.set_high()?;
        Ok(())
    }

    pub fn write_register<R>(&mut self, byte: u8) -> Result<(), E>
    where
        R: RegisterClass,
    {
        self.cs.set_low()?;
        self.spi.write(&mut [R::REGISTER_CLASS.waddr(), byte])?;
        self.cs.set_high()?;
        Ok(())
    }

    pub fn modify_register<R, F>(&mut self, f: F) -> Result<(), E>
    where
        R: RegisterClass + Copy,
        F: FnOnce(R) -> u8,
    {
        let r = self.read_register::<R>()?;
        self.write_register::<R>(f(r).into())?;
        Ok(())
    }
}
