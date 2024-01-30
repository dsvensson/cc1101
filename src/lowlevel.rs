//! Low level unrestricted access to the CC1101 radio chip.

use hal::spi::{Operation, SpiDevice};

#[macro_use]
mod macros;
mod access;
mod traits;

pub mod convert;
pub mod registers;
pub mod types;

use self::registers::*;

pub const FXOSC: u64 = 26_000_000;

pub struct Cc1101<SPI> {
    pub(crate) spi: SPI,
    //    gdo0: GDO0,
    //    gdo2: GDO2,
}

impl<SPI, SpiE> Cc1101<SPI>
where
    SPI: SpiDevice<u8, Error = SpiE>,
{
    pub fn new(spi: SPI) -> Result<Self, SpiE> {
        let cc1101 = Cc1101 {
            spi,
        };
        Ok(cc1101)
    }

    pub fn read_register<R>(&mut self, reg: R) -> Result<u8, SpiE>
    where
        R: Into<Register>,
    {
        let mut buffer = [reg.into().raddr(), 0u8];
        self.spi.transfer_in_place(&mut buffer)?;
        Ok(buffer[1])
    }

    pub fn read_fifo(&mut self, addr: &mut u8, len: &mut u8, buf: &mut [u8]) -> Result<(), SpiE> {
        let mut buffer = [Command::FIFO.addr() | 0xC0, 0, 0];

        self.spi.transaction(&mut [
            Operation::TransferInPlace(&mut buffer),
            Operation::TransferInPlace(buf),
        ])?;

        *len = buffer[1];
        *addr = buffer[2];

        Ok(())
    }

    pub fn write_strobe(&mut self, com: Command) -> Result<(), SpiE> {
        self.spi.write(&[com.addr()])?;
        Ok(())
    }

    pub fn write_register<R>(&mut self, reg: R, byte: u8) -> Result<(), SpiE>
    where
        R: Into<Register>,
    {
        self.spi.write(&[reg.into().waddr(), byte])?;
        Ok(())
    }

    pub fn modify_register<R, F>(&mut self, reg: R, f: F) -> Result<(), SpiE>
    where
        R: Into<Register> + Copy,
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_register(reg)?;
        self.write_register(reg, f(r))?;
        Ok(())
    }
}
