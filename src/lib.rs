#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Transfer, Write};
use hal::digital::v2::OutputPin;

const FXOSC: u64 = 26_000_000;

#[macro_use]
pub mod lowlevel;
mod rssi;

use lowlevel::registers::*;
use lowlevel::types::*;
use rssi::rssi_to_dbm;

/// CC1101 errors.
#[derive(Debug)]
pub enum Error<E> {
    /// The RX FIFO buffer overflowed, too small buffer for configured packet length.
    RxOverflow,
    /// Corrupt packet received with invalid CRC.
    CrcMismatch,
    /// Platform-dependent SPI-errors, such as IO errors.
    Spi(E),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Spi(e)
    }
}

/// High level API for interacting with the CC1101 radio chip.
pub struct Cc1101<SPI, CS>(lowlevel::Cc1101<SPI, CS>);

const fn deviation_to_components(v: u64) -> (u8, u8) {
    let exponent = 8 - ((v.rotate_left(14) / FXOSC) as u8).leading_zeros() - 1;
    let mantissa = (v.rotate_left(17) / (FXOSC.rotate_left(exponent))) - 7;
    ((mantissa & 0x7) as u8, (exponent & 0x7) as u8)
}

impl<SPI, CS, E> Cc1101<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin<Error = E>,
{
    pub fn new(spi: SPI, cs: CS) -> Result<Self, Error<E>> {
        Ok(Cc1101(lowlevel::Cc1101::new(spi, cs)?))
    }

    pub fn set_frequency(&mut self, hz: u64) -> Result<(), Error<E>> {
        let freq = hz * 1u64.rotate_left(16) / FXOSC;
        self.0.write_register(FREQ2(((freq >> 16) & 0xff) as u8))?;
        self.0.write_register(FREQ1(((freq >> 8) & 0xff) as u8))?;
        self.0.write_register(FREQ0((freq & 0xff) as u8))?;
        Ok(())
    }

    pub fn set_deviation(&mut self, deviation: u64) -> Result<(), Error<E>> {
        let (exponent, mantissa) = deviation_to_components(deviation);
        self.0.write_register(DEVIATN::default()
            .deviation_m(mantissa)
            .deviation_e(exponent))?;
        Ok(())
    }

    pub fn get_hw_info(&mut self) -> Result<(u8, u8), Error<E>> {
        let partnum: PARTNUM = self.0.read_register()?;
        let version: VERSION = self.0.read_register()?;
        Ok((partnum.partnum(), version.version()))
    }

    /// Received Signal Strength Indicator is an estimate of the signal power level in the chosen channel.
    pub fn get_rssi_dbm(&mut self) -> Result<i16, Error<E>> {
        let rssi: RSSI = self.0.read_register()?;
        Ok(rssi_to_dbm(rssi.into()))
    }

    /// The Link Quality Indicator metric of the current quality of the received signal.
    pub fn get_lqi(&mut self) -> Result<u8, Error<E>> {
        let lqi: LQI = self.0.read_register()?;
        Ok(lqi.lqi())
    }

    /// Configure the sync word to use, and at what level it should be verified.
    pub fn set_sync_mode(&mut self, sync_mode: SyncMode) -> Result<(), Error<E>> {
        let reset: u16 = (SYNC1::default().bits() as u16) << 8 | (SYNC0::default().bits() as u16);

        let (mode, word) = match sync_mode {
            SyncMode::Disabled => (SyncCheck::DISABLED, reset),
            SyncMode::MatchPartial(word) => (SyncCheck::CHECK_15_16, word),
            SyncMode::MatchPartialRepeated(word) => (SyncCheck::CHECK_30_32, word),
            SyncMode::MatchFull(word) => (SyncCheck::CHECK_16_16, word),
        };
        self.0.modify_register(|r: MDMCFG2| *r.modify().sync_mode(mode.value()))?;
        self.0.write_register(SYNC1(((word >> 8) & 0xff) as u8))?;
        self.0.write_register(SYNC0((word & 0xff) as u8))?;
        Ok(())
    }

    /// Configure signal modulation.
    pub fn set_modulation(&mut self, format: Modulation) -> Result<(), Error<E>> {
        use lowlevel::types::ModFormat as MF;

        let value = match format {
            Modulation::BinaryFrequencyShiftKeying => MF::MOD_2FSK,
            Modulation::GaussianFrequencyShiftKeying => MF::MOD_GFSK,
            Modulation::OnOffKeying => MF::MOD_ASK_OOK,
            Modulation::FourFrequencyShiftKeying => MF::MOD_4FSK,
            Modulation::MinimumShiftKeying => MF::MOD_MSK,
        };
        self.0.modify_register(|r: MDMCFG2| *r.modify().mod_format(value.value()))?;
        Ok(())
    }

    /// Configure device address, and address filtering.
    pub fn set_address_filter(&mut self, filter: AddressFilter) -> Result<(), Error<E>> {
        use lowlevel::types::AddressCheck as AC;

        let (mode, addr) = match filter {
            AddressFilter::Disabled => (AC::DISABLED, ADDR::default().bits()),
            AddressFilter::Device(addr) => (AC::SELF, addr),
            AddressFilter::DeviceLowBroadcast(addr) => (AC::SELF_LOW_BROADCAST, addr),
            AddressFilter::DeviceHighLowBroadcast(addr) => (AC::SELF_HIGH_LOW_BROADCAST, addr),
        };
        self.0.modify_register(|r: PKTCTRL1| *r.modify().adr_chk(mode.value()))?;
        self.0.write_register(ADDR(addr))?;
        Ok(())
    }

    /// Configure packet mode, and length.
    pub fn set_packet_length(&mut self, length: PacketLength) -> Result<(), Error<E>> {
        use lowlevel::types::LengthConfig as LC;

        let (format, pktlen) = match length {
            PacketLength::Fixed(limit) => (LC::FIXED, limit),
            PacketLength::Variable(max_limit) => (LC::VARIABLE, max_limit),
            PacketLength::Infinite => (LC::INFINITE, PKTLEN::default().bits()),
        };
        self.0.modify_register(|r: PKTCTRL0| *r.modify().length_config(format.value()))?;
        self.0.write_register(PKTLEN(pktlen))?;
        Ok(())
    }

    /// Set radio in Receive/Transmit/Idle mode.
    pub fn set_radio_mode(&mut self, radio_mode: RadioMode) -> Result<(), Error<E>> {
        let target = match radio_mode {
            RadioMode::Receive => {
                self.set_radio_mode(RadioMode::Idle)?;
                self.0.write_strobe(Command::SRX)?;
                MachineState::RX
            }
            RadioMode::Transmit => {
                self.set_radio_mode(RadioMode::Idle)?;
                self.0.write_strobe(Command::STX)?;
                MachineState::TX
            }
            RadioMode::Idle => {
                self.0.write_strobe(Command::SIDLE)?;
                MachineState::IDLE
            }
        };
        self.await_machine_state(target)
    }

    /// Configure some default settings, to be removed in the future.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn set_defaults(&mut self) -> Result<(), Error<E>> {
        self.0.write_strobe(Command::SRES)?;

        self.0.write_register(PKTCTRL0::default()
            .white_data(0)
        )?;

        self.0.write_register(FSCTRL1::default()
            .freq_if(0x08) // f_if = (f_osc / 2^10) * FREQ_IF
        )?;

        self.0.write_register(MDMCFG4::default()
            .chanbw_e(0x03) // bw_chan = f_osc / (8 * (4 + chanbw_m) * 2^chanbw_e
            .chanbw_m(0x00)
            .drate_e(0x0A)
        )?;

        self.0.write_register(MDMCFG3::default()
            .drate_m(0x83) // r_data = (((256 + drate_m) * 2^drate_e) / 2**38) * f_osc
        )?;

        self.0.write_register(MDMCFG2::default()
            .dem_dcfilt_off(1)
        )?;

        self.0.write_register(DEVIATN::default()
            .deviation_e(0x03)
            .deviation_m(0x05)
        )?;

        self.0.write_register(MCSM0::default()
            .fs_autocal(AutoCalibration::FROM_IDLE.value())
        )?;

        self.0.write_register(AGCCTRL2::default()
            .max_lna_gain(0x04)
        )?;

        Ok(())
    }

    fn await_machine_state(&mut self, target: MachineState) -> Result<(), Error<E>> {
        loop {
            let marcstate: MARCSTATE = self.0.read_register()?;
            if target.value() == marcstate.marc_state() {
                break;
            }
        }
        Ok(())
    }

    fn rx_bytes_available(&mut self) -> Result<u8, Error<E>> {
        let mut last = 0;

        loop {
            let rxbytes: RXBYTES = self.0.read_register()?;
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
        match self.rx_bytes_available() {
            Ok(_nbytes) => {
                let mut length = 0u8;
                self.0.read_fifo(addr, &mut length, buf)?;
                let lqi: LQI = self.0.read_register()?;
                self.await_machine_state(MachineState::IDLE)?;
                self.0.write_strobe(Command::SFRX)?;
                if lqi.crc_ok() != 1 {
                    Err(Error::CrcMismatch)
                } else {
                    Ok(length)
                }
            }
            Err(err) => {
                self.0.write_strobe(Command::SFRX)?;
                Err(err)
            }
        }
    }
}

/// Modulation format configuration.
pub enum Modulation {
    /// 2-FSK.
    BinaryFrequencyShiftKeying,
    /// GFSK.
    GaussianFrequencyShiftKeying,
    /// ASK / OOK.
    OnOffKeying,
    /// 4-FSK.
    FourFrequencyShiftKeying,
    /// MSK.
    MinimumShiftKeying,
}

/// Packet length configuration.
pub enum PacketLength {
    /// Set packet length to a fixed value.
    Fixed(u8),
    /// Set upper bound of variable packet length.
    Variable(u8),
    /// Infinite packet length, streaming mode.
    Infinite,
}

/// Address check configuration.
pub enum AddressFilter {
    /// No address check.
    Disabled,
    /// Address check, no broadcast.
    Device(u8),
    /// Address check and 0 (0x00) broadcast.
    DeviceLowBroadcast(u8),
    /// Address check and 0 (0x00) and 255 (0xFF) broadcast.
    DeviceHighLowBroadcast(u8),
}

/// Radio operational mode.
pub enum RadioMode {
    Receive,
    Transmit,
    Idle,
}

/// Sync word configuration.
pub enum SyncMode {
    /// No sync word.
    Disabled,
    /// Match 15 of 16 bits of given sync word.
    MatchPartial(u16),
    /// Match 30 of 32 bits of a repetition of given sync word.
    MatchPartialRepeated(u16),
    /// Match 16 of 16 bits of given sync word.
    MatchFull(u16),
}

#[cfg(test)]
mod tests {
    use ::{FXOSC, Cc1101};
    use lowlevel::registers::*;
    use deviation_to_components;

    fn calc_freq(hz: u64) -> (u8, u8, u8) {
        let freq = hz * 1u64.rotate_left(16) / FXOSC;
        ((((freq >> 16) & 0xff) as u8),
         (((freq >> 8) & 0xff) as u8),
        (((freq & 0xff) as u8)))
    }

    #[test]
    fn test_frequency() {
        assert_eq!(calc_freq(433_000_000), (0x10, 0xA7, 0x62));
        assert_eq!(calc_freq(868_000_000), (0x21, 0x62, 0x76));
        assert_eq!(calc_freq(902_000_000), (0x22, 0xB1, 0x3B));
        assert_eq!(calc_freq(918_000_000), (0x23, 0x4E, 0xC4));
    }

    #[test]
    fn test_drate() {
        // DRATE = 1000000.0 * MHZ * (256+drate_m) * powf(2,drate_e) / powf(2,28);
        // 0xF8 - MDMCFG4       Modem Configuration - BW: 58.035Khz (0xF6 would give 2.4kBaud
        // 0x83 - MDMCFG3       Modem Configuration - 9595 Baud

        assert_eq!((8,131), (MDMCFG4(0xF8).drate_e(), MDMCFG3(0x83).drate_m()));

        assert_eq!((9595u64.rotate_left(28) / FXOSC.rotate_left(8)) - 256, 131);
        assert_eq!(8 - ((9595u64.rotate_left(20) / FXOSC) as u8).leading_zeros() - 1, 8)

        //radio_parms->drate_e = (uint8_t) (floor(log2( drate*(1<<20) / f_xtal )));
        //radio_parms->drate_m = (uint8_t) ( ((drate*(1<<28)) / (f_xtal * (1<<radio_parms->drate_e))) - 256);


    }

    #[test]
    fn test_deviation() {
        fn calc_rev_dev(dev_m: u8, dev_e: u8) -> u64 {
            (((FXOSC as f32 / (2u64.pow(17) as f32)) as f32) * (8f32 + dev_m as f32) * (2u64.pow(dev_e as u32) as f32)) as u64
        }

        for e in 0..7 {
            for m in 1..7 {
                assert_eq!(deviation_to_components(calc_rev_dev(m, e)), (m, e));
            }
        }
    }

    /*
    fn calc_chanspc(chanspc: u64) -> (u8, u8) {

    }

    #[test]
    fn test_foo() {
        assert_eq!((0,0), MDMCFG1(0x22).chanspc_e(), MDMCFG0(0xF8).chanspc_m())
    }*/
}
