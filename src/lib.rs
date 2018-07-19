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
pub mod lowlevel;
mod rssi;

use lowlevel::types::*;
use rssi::rssi_to_dbm;

#[derive(Debug)]
pub enum Error<E> {
    RxOverflow,
    CrcMismatch,
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
        self.write_register(lowlevel::Config::FREQ2, ((freq >> 16) & 0xff) as u8)?;
        self.write_register(lowlevel::Config::FREQ1, ((freq >> 8) & 0xff) as u8)?;
        self.write_register(lowlevel::Config::FREQ0, (freq & 0xff) as u8)?;
        Ok(())
    }

    pub fn get_hw_info(&mut self) -> Result<(u8, u8), Error<E>> {
        let partnum = self.read_register(lowlevel::Status::PARTNUM)?;
        let version = self.read_register(lowlevel::Status::VERSION)?;
        Ok((partnum, version))
    }

    pub fn get_rssi_dbm(&mut self) -> Result<i16, Error<E>> {
        Ok(rssi_to_dbm(self.read_register(lowlevel::Status::RSSI)?))
    }

    pub fn get_lqi(&mut self) -> Result<u8, Error<E>> {
        let lqi = self.read_register(lowlevel::Status::LQI)?;
        Ok(lqi & !(1u8 << 7))
    }

    pub fn set_sync_mode(&mut self, sync_mode: SyncMode) -> Result<(), Error<E>> {
        use lowlevel::*;

        let reset: u16 = (SYNC1::default().bits() as u16) << 8 | (SYNC0::default().bits() as u16);

        let (mode, word) = match sync_mode {
            SyncMode::Disabled => (SyncCheck::DISABLED, reset),
            SyncMode::MatchPartial(word) => (SyncCheck::CHECK_15_16, word),
            SyncMode::MatchPartialRepeated(word) => (SyncCheck::CHECK_30_32, word),
            SyncMode::MatchFull(word) => (SyncCheck::CHECK_16_16, word),
        };
        self.modify_register(Config::MDMCFG2, |r| {
            MDMCFG2(r).modify().sync_mode(mode.value()).bits()
        })?;
        self.write_register(Config::SYNC1, ((word >> 8) & 0xff) as u8)?;
        self.write_register(Config::SYNC0, (word & 0xff) as u8)
    }

    pub fn set_modulation(&mut self, format: Modulation) -> Result<(), Error<E>> {
        use lowlevel::types::ModFormat as MF;
        use lowlevel::*;

        let value = match format {
            Modulation::BinaryFrequencyShiftKeying => MF::MOD_2FSK,
            Modulation::GaussianFrequencyShiftKeying => MF::MOD_GFSK,
            Modulation::OnOffKeying => MF::MOD_ASK_OOK,
            Modulation::FourFrequencyShiftKeying => MF::MOD_4FSK,
            Modulation::MinimumShiftKeying => MF::MOD_MSK,
        };
        self.modify_register(Config::MDMCFG2, |r| {
            MDMCFG2(r).modify().mod_format(value.value()).bits()
        })
    }

    pub fn set_address_filter(&mut self, filter: AddressFilter) -> Result<(), Error<E>> {
        use lowlevel::types::AddressCheck as AC;
        use lowlevel::*;

        let (mode, addr) = match filter {
            AddressFilter::Disabled => (AC::DISABLED, ADDR::default().bits()),
            AddressFilter::Device(addr) => (AC::SELF, addr),
            AddressFilter::DeviceLowBroadcast(addr) => (AC::SELF_LOW_BROADCAST, addr),
            AddressFilter::DeviceHighLowBroadcast(addr) => (AC::SELF_HIGH_LOW_BROADCAST, addr),
        };
        self.modify_register(Config::PKTCTRL1, |r| {
            PKTCTRL1(r).modify().adr_chk(mode.value()).bits()
        })?;
        self.write_register(Config::ADDR, addr)
    }

    pub fn set_packet_length(&mut self, length: PacketLength) -> Result<(), Error<E>> {
        use lowlevel::types::LengthConfig as LC;
        use lowlevel::*;

        let (format, pktlen) = match length {
            PacketLength::Fixed(limit) => (LC::FIXED, limit),
            PacketLength::Variable(max_limit) => (LC::VARIABLE, max_limit),
            PacketLength::Infinite => (LC::INFINITE, PKTLEN::default().bits()),
        };
        self.modify_register(Config::PKTCTRL0, |r| {
            PKTCTRL0(r).modify().length_config(format.value()).bits()
        })?;
        self.write_register(Config::PKTLEN, pktlen)
    }

    pub fn set_radio_mode(&mut self, radio_mode: RadioMode) -> Result<(), Error<E>> {
        use lowlevel::*;

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
        self.await_machine_state(target)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn set_defaults(&mut self) -> Result<(), Error<E>> {
        use lowlevel::*;

        self.write_strobe(Command::SRES)?;

        self.write_register(Config::PKTCTRL0, PKTCTRL0::default()
            .white_data(0).bits()
        )?;

        self.write_register(Config::FSCTRL1, FSCTRL1::default()
            .freq_if(0x08).bits() // f_if = (f_osc / 2^10) * FREQ_IF
        )?;

        self.write_register(Config::MDMCFG4, MDMCFG4::default()
            .chanbw_e(0x03) // bw_chan = f_osc / (8 * (4 + chanbw_m) * 2^chanbw_e
            .chanbw_m(0x00)
            .drate_e(0x0A).bits()
        )?;

        self.write_register(Config::MDMCFG3, MDMCFG3::default()
            .drate_m(0x83).bits() // r_data = (((256 + drate_m) * 2^drate_e) / 2**38) * f_osc
        )?;

        self.write_register(Config::MDMCFG2, MDMCFG2::default()
            .dem_dcfilt_off(1).bits()
        )?;

        self.write_register(Config::DEVIATN, DEVIATN::default()
            .deviation_e(0x03)
            .deviation_m(0x05).bits()
        )?;

        self.write_register(Config::MCSM0, MCSM0::default()
            .fs_autocal(AutoCalibration::FROM_IDLE.value()).bits()
        )?;

        self.write_register(Config::AGCCTRL2, AGCCTRL2::default()
            .max_lna_gain(0x04).bits()
        )?;

        Ok(())
    }

    fn await_machine_state(&mut self, target: MachineState) -> Result<(), Error<E>> {
        use lowlevel::*;
        loop {
            let marcstate = MARCSTATE(self.read_register(Status::MARCSTATE)?);
            if target.value() == marcstate.marc_state() {
                break;
            }
        }
        Ok(())
    }

    fn rx_bytes_available(&mut self) -> Result<u8, Error<E>> {
        use lowlevel::*;

        let mut last = 0;

        loop {
            let rxbytes = RXBYTES(self.read_register(Status::RXBYTES)?);
            if rxbytes.rxfifo_overflow() == 1 {
                return Err(Error::RxOverflow);
            }

            let nbytes = rxbytes.num_rxbytes();
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
    pub fn receive(&mut self, addr: &mut u8, buf: &mut [u8]) -> Result<u8, Error<E>> {
        use lowlevel::*;

        match self.rx_bytes_available() {
            Ok(_nbytes) => {
                let mut length = 0u8;
                self.read_fifo(addr, &mut length, buf)?;
                let lqi = self.read_register(Status::LQI)?;
                self.await_machine_state(MachineState::IDLE)?;
                self.write_strobe(Command::SFRX)?;
                if (lqi >> 7) != 1 {
                    Err(Error::CrcMismatch)
                } else {
                    Ok(length)
                }
            }
            Err(err) => {
                self.write_strobe(Command::SFRX)?;
                Err(err)
            }
        }
    }

    fn read_register<R>(&mut self, reg: R) -> Result<u8, Error<E>>
    where
        R: Into<lowlevel::Register>,
    {
        self.cs.set_low();
        let mut buffer = [reg.into().raddr(), 0u8];
        self.spi.transfer(&mut buffer)?;
        self.cs.set_high();
        Ok(buffer[1])
    }

    fn read_fifo(&mut self, addr: &mut u8, len: &mut u8, buf: &mut [u8]) -> Result<(), Error<E>> {
        let mut buffer = [lowlevel::Command::FIFO.addr() | 0xC0, 0, 0];

        self.cs.set_low();
        self.spi.transfer(&mut buffer)?;
        self.spi.transfer(buf)?;
        self.cs.set_high();

        *len = buffer[1];
        *addr = buffer[2];

        Ok(())
    }

    fn write_strobe(&mut self, com: lowlevel::Command) -> Result<(), Error<E>> {
        self.cs.set_low();
        self.spi.write(&[com.addr()])?;
        self.cs.set_high();
        Ok(())
    }

    fn write_register<R>(&mut self, reg: R, byte: u8) -> Result<(), Error<E>>
    where
        R: Into<lowlevel::Register>,
    {
        self.cs.set_low();
        self.spi.write(&mut [reg.into().waddr(), byte])?;
        self.cs.set_high();
        Ok(())
    }

    fn modify_register<R, F>(&mut self, reg: R, f: F) -> Result<(), Error<E>>
    where
        R: Into<lowlevel::Register> + Copy,
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_register(reg)?;
        self.write_register(reg, f(r))?;
        Ok(())
    }
}

pub enum Modulation {
    BinaryFrequencyShiftKeying,
    GaussianFrequencyShiftKeying,
    OnOffKeying,
    FourFrequencyShiftKeying,
    MinimumShiftKeying,
}

pub enum PacketLength {
    Fixed(u8),
    Variable(u8),
    Infinite,
}

pub enum AddressFilter {
    Disabled,
    Device(u8),
    DeviceLowBroadcast(u8),
    DeviceHighLowBroadcast(u8),
}

pub enum RadioMode {
    Receive,
    Transmit,
    Idle,
}

pub enum SyncMode {
    Disabled,
    MatchPartial(u16),
    MatchPartialRepeated(u16),
    MatchFull(u16),
}
