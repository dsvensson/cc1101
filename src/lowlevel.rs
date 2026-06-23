//! Low level unrestricted access to the CC1101 radio chip.

use hal::spi::{Operation, SpiDevice};

mod traits;

pub mod access;
pub mod convert;
pub mod registers;
pub mod types;

use self::registers::{BurstRead, BurstWrite, Readable, StatusByte, Strobe, Writable};

pub const FXOSC: u64 = 26_000_000;
pub const FIFO_SIZE_MAX: u8 = 64;
const BLANK_BYTE: u8 = 0;

pub struct Cc1101<SPI> {
    pub(crate) spi: SPI,
    pub status: Option<StatusByte>,
    pub length_field: bool,
    pub address_field: bool,
    pub rx_status_fields: bool,
}

impl<SPI, SpiE> Cc1101<SPI>
where
    SPI: SpiDevice<u8, Error = SpiE>,
{
    pub fn new(spi: SPI) -> Result<Self, SpiE> {
        let cc1101 = Cc1101 {
            spi,
            status: None,
            length_field: false,
            address_field: false,
            rx_status_fields: true,
        };
        Ok(cc1101)
    }

    /// Read a single-byte register, returning its typed read view.
    pub fn read_register<S: Readable>(&mut self, _reg: S) -> Result<S::View, SpiE> {
        let mut buffer = [access::Access::Read as u8 | S::MODE as u8 | S::ADDR, BLANK_BYTE];
        self.spi.transfer_in_place(&mut buffer)?;
        self.status = Some(StatusByte::from(buffer[0]));
        Ok(S::view(buffer[1]))
    }

    /// Write a single-byte register, building its value from the reset state (no readback).
    pub fn write_register<S, F>(&mut self, _reg: S, f: F) -> Result<(), SpiE>
    where
        S: Writable,
        F: FnOnce(S::View) -> S::View,
    {
        let byte = S::bits(f(S::view(S::RESET)));
        let mut buffer = [access::Access::Write as u8 | S::MODE as u8 | S::ADDR, byte];
        self.spi.transfer_in_place(&mut buffer)?;
        self.status = Some(StatusByte::from(buffer[0]));
        Ok(())
    }

    /// Read-modify-write a single-byte register.
    pub fn modify_register<S, F>(&mut self, _reg: S, f: F) -> Result<(), SpiE>
    where
        S: Readable + Writable,
        F: FnOnce(<S as Writable>::View) -> <S as Writable>::View,
    {
        let mut rbuffer = [
            access::Access::Read as u8 | <S as Readable>::MODE as u8 | <S as Readable>::ADDR,
            BLANK_BYTE,
        ];
        self.spi.transfer_in_place(&mut rbuffer)?;
        self.status = Some(StatusByte::from(rbuffer[0]));

        let byte = <S as Writable>::bits(f(<S as Writable>::view(rbuffer[1])));
        let mut wbuffer = [
            access::Access::Write as u8 | <S as Writable>::MODE as u8 | <S as Writable>::ADDR,
            byte,
        ];
        self.spi.transfer_in_place(&mut wbuffer)?;
        self.status = Some(StatusByte::from(wbuffer[0]));
        Ok(())
    }

    /// Fire a command strobe and return the chip status byte returned during it.
    pub fn strobe<S: Strobe>(&mut self, _cmd: S) -> Result<StatusByte, SpiE> {
        let mut buffer = [access::Access::Write as u8 | access::Mode::Single as u8 | S::ADDR];
        self.spi.transfer_in_place(&mut buffer)?;
        let status = StatusByte::from(buffer[0]);
        if S::NO_EFFECT {
            // SNOP is the only command with no effect and therefore can be used to
            // get access to the chip status byte.
            self.status = Some(status);
        } else {
            // Discard the returned status byte: for a state-changing strobe it most
            // probably reflects the previous state. Set `None` to inform the user of
            // the lack of a valid state read.
            self.status = None;
        }
        Ok(status)
    }

    /// Burst-read a multi-byte region (e.g. PATABLE) into `buf`.
    pub fn read_burst<S: BurstRead>(&mut self, _reg: S, buf: &mut [u8]) -> Result<(), SpiE> {
        let mut header = [access::Access::Read as u8 | access::Mode::Burst as u8 | S::ADDR];
        self.spi
            .transaction(&mut [Operation::TransferInPlace(&mut header), Operation::Read(buf)])?;
        self.status = Some(StatusByte::from(header[0]));
        Ok(())
    }

    /// Burst-write a multi-byte region (e.g. PATABLE) from `data`.
    pub fn write_burst<S: BurstWrite>(&mut self, _reg: S, data: &[u8]) -> Result<(), SpiE> {
        let mut header = [access::Access::Write as u8 | access::Mode::Burst as u8 | S::ADDR];
        self.spi
            .transaction(&mut [Operation::TransferInPlace(&mut header), Operation::Write(data)])?;
        self.status = Some(StatusByte::from(header[0]));
        Ok(())
    }

    /// Framed FIFO access: header + optional length/address fields + data, in one transaction.
    pub fn access_fifo(
        &mut self,
        access: access::Access,
        optional_fields: &mut [u8],
        data: &mut [u8],
    ) -> Result<(), SpiE> {
        let mut buffer = [access as u8
            | access::Mode::Burst as u8
            | <registers::multi::FIFO as BurstRead>::ADDR];

        if optional_fields.is_empty() {
            self.spi.transaction(&mut [
                Operation::TransferInPlace(&mut buffer),
                Operation::TransferInPlace(data),
            ])?;
        } else {
            self.spi.transaction(&mut [
                Operation::TransferInPlace(&mut buffer),
                Operation::TransferInPlace(optional_fields),
                Operation::TransferInPlace(data),
            ])?;
        }

        self.status = Some(StatusByte::from(buffer[0]));
        Ok(())
    }
}
