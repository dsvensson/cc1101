//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Transfer, Write};
//use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;

const FXOSC: u64 = 26_000_000;

#[macro_use]
mod macros;
mod config;
mod traits;

#[derive(Debug)]
pub enum Error<E> {
    RxOverflow,
    Spi(E),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Spi(e)
    }
}

pub struct Cc1101<SPI, CS> {
    spi: SPI,
    cs: CS,
    //    gdo0: GDO0,
    //    gdo2: GDO2,
}

impl<SPI, CS, E> Cc1101<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, Error<E>> {
        let cc1101 = Cc1101 { spi: spi, cs: cs };

        Ok(cc1101)
    }

    pub fn set_frequency(&mut self, hz: u64) -> Result<(), Error<E>> {
        let freq = hz * 1u64.rotate_left(16) / FXOSC;
        self.write_register(config::Register::FREQ2, ((freq >> 16) & 0xff) as u8)?;
        self.write_register(config::Register::FREQ1, ((freq >> 8) & 0xff) as u8)?;
        self.write_register(config::Register::FREQ0, (freq & 0xff) as u8)?;
        Ok(())
    }

    pub fn set_sync_mode(&mut self, sync_mode: u8) -> Result<(), Error<E>> {
        self.modify_register(config::Register::MDMCFG2, |r| {
            (r & 0b11111000) | (sync_mode & 0b111)
        })?;
        Ok(())
    }

    pub fn set_sync_word(&mut self, sync_word: u16) -> Result<(), Error<E>> {
        self.write_register(config::Register::SYNC1, ((sync_word >> 8) & 0xff) as u8)?;
        self.write_register(config::Register::SYNC0, (sync_word & 0xff) as u8)?;
        Ok(())
    }

    pub fn set_modulation(&mut self, sync_mode: Modulation) -> Result<(), Error<E>> {
        self.modify_register(config::Register::MDMCFG2, |r| {
            (r & 0b10001111) | (sync_mode.addr() << 4)
        })?;
        Ok(())
    }

    pub fn set_packet_length(&mut self, length: PacketLength) -> Result<(), Error<E>> {
        match length {
            PacketLength::Fixed(limit) => {
                self.modify_register(config::Register::PKTCTRL0, |r| r & 0b00111111)?;
                self.write_register(config::Register::PKTLEN, limit)?;
            }
            PacketLength::Variable(max_limit) => {
                self.modify_register(config::Register::PKTCTRL0, |r| {
                    (r & 0b00111111) | (0b01 << 6)
                })?;
                self.write_register(config::Register::PKTLEN, max_limit)?;
            }
            PacketLength::Infinite => {
                let reset: u8 = 0xff;
                self.modify_register(config::Register::PKTCTRL0, |r| {
                    (r & 0b00111111) | (0b11 << 6)
                })?;
                self.write_register(config::Register::PKTLEN, reset)?;
            }
        }
        Ok(())
    }

    pub fn set_radio_mode(&mut self, radio_mode: RadioMode) -> Result<(), Error<E>> {
        let target = match radio_mode {
            RadioMode::Receive => {
                self.set_radio_mode(RadioMode::Idle)?;
                self.write_strobe(Command::SRX)?;
                MachineState::RX
            }
            RadioMode::Transmit => {
                self.set_radio_mode(RadioMode::Idle)?;
                self.write_strobe(Command::STX)?;
                MachineState::TX
            }
            RadioMode::Idle => {
                self.write_strobe(Command::SIDLE)?;
                MachineState::IDLE
            }
        };
        self.await_machine_state(target)?;
        Ok(())
    }

    pub fn set_defaults(&mut self) -> Result<(), Error<E>> {
        // Default values extracted from Smart RF Studio 7
        // Should be replaced with calls to properly named
        // functions.
        self.write_register(config::Register::IOCFG2, 0x2E)?;
        self.write_register(config::Register::IOCFG1, 0x2E)?;
        self.write_register(config::Register::IOCFG0, 0x06)?;
        self.write_register(config::Register::FIFOTHR, 0x07)?;
        self.write_register(config::Register::PKTLEN, 20)?;
        self.write_register(config::Register::PKTCTRL1, 0x06)?;
        self.write_register(config::Register::PKTCTRL0, 0x04)?;
        self.write_register(config::Register::CHANNR, 0x00)?;
        self.modify_register(config::Register::PKTCTRL1, |r| r & 0b11111100)?;
        self.write_register(config::Register::FSCTRL1, 0x08)?;
        self.write_register(config::Register::FSCTRL0, 0x00)?;
        self.write_register(config::Register::MDMCFG4, 0xCA)?;
        self.write_register(config::Register::MDMCFG3, 0x83)?;
        self.write_register(config::Register::MDMCFG2, 0x93)?;
        self.write_register(config::Register::MDMCFG1, 0x22)?;
        self.write_register(config::Register::MDMCFG0, 0xF8)?;
        self.write_register(config::Register::DEVIATN, 0x35)?;
        self.write_register(config::Register::MCSM2, 0x07)?;
        self.write_register(config::Register::MCSM1, 0x20)?;
        self.write_register(config::Register::MCSM0, 0x18)?;
        self.write_register(config::Register::FOCCFG, 0x16)?;
        self.write_register(config::Register::BSCFG, 0x6C)?;
        self.write_register(config::Register::AGCCTRL2, 0x43)?;
        self.write_register(config::Register::AGCCTRL1, 0x40)?;
        self.write_register(config::Register::AGCCTRL0, 0x91)?;
        self.write_register(config::Register::WOREVT1, 0x87)?;
        self.write_register(config::Register::WOREVT0, 0x6B)?;
        self.write_register(config::Register::WORCTRL, 0xFB)?;
        self.write_register(config::Register::FREND1, 0x56)?;
        self.write_register(config::Register::FREND0, 0x10)?;
        self.write_register(config::Register::FSCAL3, 0xE9)?;
        self.write_register(config::Register::FSCAL2, 0x2A)?;
        self.write_register(config::Register::FSCAL1, 0x00)?;
        self.write_register(config::Register::FSCAL0, 0x1F)?;
        self.write_register(config::Register::RCCTRL1, 0x41)?;
        self.write_register(config::Register::RCCTRL0, 0x00)?;
        self.write_register(config::Register::FSTEST, 0x59)?;
        self.write_register(config::Register::PTEST, 0x7F)?;
        self.write_register(config::Register::AGCTEST, 0x3F)?;
        self.write_register(config::Register::TEST2, 0x81)?;
        self.write_register(config::Register::TEST1, 0x35)?;
        self.write_register(config::Register::TEST0, 0x09)?;
        //self.write_register(config::Register::PATABLE, 0xC0)?;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_strobe(Command::SRES)?;
        Ok(())
    }

    fn await_machine_state(&mut self, target: MachineState) -> Result<(), Error<E>> {
        let mut marcstate = [0u8; 2];
        loop {
            self.read_burst(Command::STX, &mut marcstate)?;
            if target.value() == (marcstate[1] & 0b11111) {
                break;
            }
        }
        Ok(())
    }

    pub fn rx_bytes_available(&mut self) -> Result<u8, Error<E>> {
        let num_bytes_mask = 0x7F;
        let overflow = 1 << 7;

        let mut last = 0;
        let mut rxbytes = [0u8; 2];
        loop {
            self.read_burst(Command::SFTX, &mut rxbytes)?;
            if (rxbytes[1] & overflow) > 0 {
                return Err(Error::RxOverflow);
            }
            let mut nbytes = rxbytes[1] & num_bytes_mask;
            if nbytes > 0 && nbytes == last {
                break;
            }
            last = nbytes;
        }
        Ok(last)
    }

    // Should also be able to configure MCSM1.RXOFF_MODE to declare what state
    // to enter after fully receiving a packet.
    // Possible targets: IDLE, FSTON, TX, RX
    pub fn receive(&mut self, buf: &mut [u8], rssi: &mut u8, lsi: &mut u8) -> Result<(), Error<E>> {
        let _nbytes = self.rx_bytes_available()?;

        self.read_burst(Command::FIFO, buf)?;

        // ugh.. to move..
        {
            let mut status = [Command::FIFO.addr() | Access::READ_SINGLE.offset(), 0];
            self.cs.set_low();
            self.spi.transfer(&mut status)?;
            self.cs.set_high();
            *rssi = status[1];
        }

        {
            let mut status = [Command::FIFO.addr() | Access::READ_SINGLE.offset(), 0];
            self.cs.set_low();
            self.spi.transfer(&mut status)?;
            self.cs.set_high();
            *lsi = status[1];
        }

        self.write_strobe(Command::SFRX)?;

        Ok(())
    }

    fn read_register(&mut self, reg: config::Register) -> Result<u8, Error<E>> {
        self.cs.set_low();

        let mut buffer = [reg.addr() | Access::READ_SINGLE.offset(), 0];
        self.spi.transfer(&mut buffer)?;

        self.cs.set_high();

        Ok(buffer[1])
    }

    fn read_burst(&mut self, com: Command, buf: &mut [u8]) -> Result<(), Error<E>> {
        self.cs.set_low();
        buf[0] = com.addr() | Access::READ_BURST.offset();
        self.spi.transfer(buf)?;
        self.cs.set_high();
        Ok(())
    }

    fn write_strobe(&mut self, com: Command) -> Result<(), Error<E>> {
        self.cs.set_low();
        self.spi.write(&[com.addr()])?;
        self.cs.set_high();
        Ok(())
    }

    fn write_register(&mut self, reg: config::Register, byte: u8) -> Result<(), Error<E>> {
        self.cs.set_low();

        let mut buffer = [reg.addr() | Access::WRITE_SINGLE.offset(), byte];
        self.spi.write(&mut buffer)?;

        self.cs.set_high();

        Ok(())
    }

    fn write_burst(&mut self, com: Command, buf: &mut [u8]) -> Result<(), Error<E>> {
        self.cs.set_low();

        // Hopefully the same as writing an array that starts with the command followed by buf
        self.spi
            .write(&[com.addr() | Access::WRITE_BURST.offset()])?;
        self.spi.write(&buf)?;

        self.cs.set_high();

        Ok(())
    }

    fn modify_register<F>(&mut self, reg: config::Register, f: F) -> Result<(), Error<E>>
    where
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_register(reg)?;
        self.write_register(reg, f(r))?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Access {
    /// Write Single Byte
    WRITE_SINGLE = 0x00,
    /// Write Burst
    WRITE_BURST = 0x40,
    /// Read Single Byte
    READ_SINGLE = 0x80,
    /// Read Burst
    READ_BURST = 0xC0,
}

impl Access {
    fn offset(&self) -> u8 {
        *self as u8
    }
}

impl Command {
    fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Command {
    /* STROBE COMMANDS */
    SRES = 0x30,    // Reset chip
    SFSTXON = 0x31, // Enable/calibrate freq synthesizer
    SXOFF = 0x32,   // Turn off crystal oscillator.
    SCAL = 0x33,    // Calibrate freq synthesizer & disable
    SRX = 0x34,     // Enable RX.
    STX = 0x35,     // Enable TX.
    SIDLE = 0x36,   // Exit RX / TX
    SAFC = 0x37,    // AFC adjustment of freq synthesizer
    SWOR = 0x38,    // Start automatic RX polling sequence
    SPWD = 0x39,    // Enter pwr down mode when CSn goes hi
    SFRX = 0x3A,    // Flush the RX FIFO buffer.
    SFTX = 0x3B,    // Flush the TX FIFO buffer.
    SWORRST = 0x3C, // Reset real time clock.
    SNOP = 0x3D,    // No operation.
    PATABLE = 0x3E, // Power Amplifier Table
    FIFO = 0x3F,    // FIFO Access
}

impl Modulation {
    fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(non_camel_case_types)]
pub enum Modulation {
    MOD_2FSK = 0b000,
    MOD_GFSK = 0b001,
    MOD_ASK_OOK = 0b011,
    MOD_4FSK = 0b100,
    MOD_MSK = 0b111,
}

pub enum PacketLength {
    Fixed(u8),
    Variable(u8),
    Infinite,
}

pub enum RadioMode {
    Receive,
    Transmit,
    Idle,
}

impl MachineState {
    fn value(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum MachineState {
    SLEEP = 0x00,
    IDLE = 0x01,
    XOFF = 0x02,
    VCOON_MC = 0x03,
    REGON_MC = 0x04,
    MANCAL = 0x05,
    VCOON = 0x06,
    REGON = 0x07,
    STARTCAL = 0x08,
    BWBOOST = 0x09,
    FS_LOCK = 0x0A,
    IFADCON = 0x0B,
    ENDCAL = 0x0C,
    RX = 0x0D,
    RX_END = 0x0E,
    RX_RST = 0x0F,
    TXRX_SWITCH = 0x10,
    RXFIFO_OVERFLOW = 0x11,
    FSTXON = 0x12,
    TX = 0x13,
    TX_END = 0x14,
    RXTX_SWITCH = 0x15,
    TXFIFO_UNDERFLOW = 0x16,
}
