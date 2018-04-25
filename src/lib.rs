//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Transfer, Write};
//use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;

const FXOSC: u64 = 26_000_000;

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
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let cc1101 = Cc1101 { spi: spi, cs: cs };

        Ok(cc1101)
    }

    pub fn set_frequency(&mut self, hz: u64) -> Result<(), E> {
        let freq = hz * 1u64.rotate_left(16) / FXOSC;
        self.write_register(Register::FREQ2, ((freq >> 16) & 0xff) as u8)?;
        self.write_register(Register::FREQ1, ((freq >> 8) & 0xff) as u8)?;
        self.write_register(Register::FREQ0, (freq & 0xff) as u8)?;
        Ok(())
    }

    pub fn set_sync_mode(&mut self, sync_mode: u8) -> Result<(), E> {
        self.modify_register(Register::MDMCFG2, |r| {
            (r & 0b11111000) | (sync_mode & 0b111)
        })?;
        Ok(())
    }

    pub fn set_sync_word(&mut self, sync_word: u16) -> Result<(), E> {
        self.write_register(Register::SYNC1, ((sync_word >> 8) & 0xff) as u8)?;
        self.write_register(Register::SYNC0, (sync_word & 0xff) as u8)?;
        Ok(())
    }

    pub fn set_modulation(&mut self, sync_mode: Modulation) -> Result<(), E> {
        self.modify_register(Register::MDMCFG2, |r| {
            (r & 0b10001111) | (sync_mode.addr() << 4)
        })?;
        Ok(())
    }

    pub fn set_packet_length(&mut self, length: PacketLength) -> Result<(), E> {
        match length {
            PacketLength::Fixed(limit) => {
                self.modify_register(Register::PKTCTRL0, |r| r & 0b00111111)?;
                self.write_register(Register::PKTLEN, limit)?;
            }
            PacketLength::Variable(max_limit) => {
                self.modify_register(Register::PKTCTRL0, |r| (r & 0b00111111) | (0b01 << 6))?;
                self.write_register(Register::PKTLEN, max_limit)?;
            }
            PacketLength::Infinite => {
                let reset: u8 = 0xff;
                self.modify_register(Register::PKTCTRL0, |r| (r & 0b00111111) | (0b11 << 6))?;
                self.write_register(Register::PKTLEN, reset)?;
            }
        }
        Ok(())
    }

    pub fn set_radio_mode(&mut self, radio_mode: RadioMode) -> Result<(), E> {
        match radio_mode {
            RadioMode::Receive => {
                self.write_strobe(Command::SIDLE)?;
                self.write_strobe(Command::SRX)?;
            }
            RadioMode::Transmit => {
                self.write_strobe(Command::SIDLE)?;
                self.write_strobe(Command::STX)?;
            }
            RadioMode::Idle => self.write_strobe(Command::SIDLE)?,
        };
        // while self.read_register(Register::MARCSTATE) RX: 0x0d, TX: 0x1f, and maybe delay
        Ok(())
    }

    pub fn set_defaults(&mut self) -> Result<(), E> {
        // Default values extracted from Smart RF Studio 7
        // Should be replaced with calls to properly named
        // functions.
        self.write_register(Register::IOCFG2, 0x2E)?;
        self.write_register(Register::IOCFG1, 0x2E)?;
        self.write_register(Register::IOCFG0, 0x06)?;
        self.write_register(Register::FIFOTHR, 0x07)?;
        self.write_register(Register::PKTLEN, 20)?;
        self.write_register(Register::PKTCTRL1, 0x06)?;
        self.write_register(Register::PKTCTRL0, 0x04)?;
        self.write_register(Register::CHANNR, 0x00)?;
        self.modify_register(Register::PKTCTRL1, |r| r & 0b11111100)?;
        self.write_register(Register::FSCTRL1, 0x08)?;
        self.write_register(Register::FSCTRL0, 0x00)?;
        self.write_register(Register::MDMCFG4, 0xCA)?;
        self.write_register(Register::MDMCFG3, 0x83)?;
        self.write_register(Register::MDMCFG2, 0x93)?;
        self.write_register(Register::MDMCFG1, 0x22)?;
        self.write_register(Register::MDMCFG0, 0xF8)?;
        self.write_register(Register::DEVIATN, 0x35)?;
        self.write_register(Register::MCSM2, 0x07)?;
        self.write_register(Register::MCSM1, 0x20)?;
        self.write_register(Register::MCSM0, 0x18)?;
        self.write_register(Register::FOCCFG, 0x16)?;
        self.write_register(Register::BSCFG, 0x6C)?;
        self.write_register(Register::AGCCTRL2, 0x43)?;
        self.write_register(Register::AGCCTRL1, 0x40)?;
        self.write_register(Register::AGCCTRL0, 0x91)?;
        self.write_register(Register::WOREVT1, 0x87)?;
        self.write_register(Register::WOREVT0, 0x6B)?;
        self.write_register(Register::WORCTRL, 0xFB)?;
        self.write_register(Register::FREND1, 0x56)?;
        self.write_register(Register::FREND0, 0x10)?;
        self.write_register(Register::FSCAL3, 0xE9)?;
        self.write_register(Register::FSCAL2, 0x2A)?;
        self.write_register(Register::FSCAL1, 0x00)?;
        self.write_register(Register::FSCAL0, 0x1F)?;
        self.write_register(Register::RCCTRL1, 0x41)?;
        self.write_register(Register::RCCTRL0, 0x00)?;
        self.write_register(Register::FSTEST, 0x59)?;
        self.write_register(Register::PTEST, 0x7F)?;
        self.write_register(Register::AGCTEST, 0x3F)?;
        self.write_register(Register::TEST2, 0x81)?;
        self.write_register(Register::TEST1, 0x35)?;
        self.write_register(Register::TEST0, 0x09)?;
        self.write_register(Register::PATABLE, 0xC0)?;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), E> {
        self.write_strobe(Command::SRES)?;
        Ok(())
    }

    fn await_machine_state(&mut self, target: MachineState) -> Result<(), E> {
        loop {
            let state = self.read_register(Register::MARCSTATE)? & 0b11111;
            if target.value() == state {
                break;
            }
        }
        Ok(())
    }

    pub fn receive_would_block(&mut self) -> Result<bool, E> {
        let rx_bytes = self.read_register(Register::RXBYTES)?;
        Ok(!((rx_bytes & 0x7F > 0) && (rx_bytes & 0x80 == 0)))
    }

    pub fn receive(&mut self, buf: &mut [u8], rssi: &mut u8, lsi: &mut u8) -> Result<(), E> {
        while self.receive_would_block()? {}

        self.read_burst(Command::RXFIFO_BURST, buf)?;

        // ugh.. to move..
        {
            let mut status = [Command::TXFIFO_SINGLE_BYTE.addr() | READ_SINGLE_BYTE, 0];
            self.cs.set_low();
            self.spi.transfer(&mut status)?;
            self.cs.set_high();
            *rssi = status[1];
        }

        {
            let mut status = [Command::TXFIFO_SINGLE_BYTE.addr() | READ_SINGLE_BYTE, 0];
            self.cs.set_low();
            self.spi.transfer(&mut status)?;
            self.cs.set_high();
            *lsi = status[1];
        }

        self.write_strobe(Command::SFRX)?;

        Ok(())
    }

    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        self.cs.set_low();

        let mut buffer = [reg.addr() | READ_SINGLE_BYTE, 0];
        self.spi.transfer(&mut buffer)?;

        self.cs.set_high();

        Ok(buffer[1])
    }

    fn read_burst(&mut self, com: Command, buf: &mut [u8]) -> Result<(), E> {
        self.cs.set_low();
        buf[0] = com.addr() | READ_BURST;
        self.spi.transfer(buf)?;
        self.cs.set_high();
        Ok(())
    }

    fn write_strobe(&mut self, com: Command) -> Result<(), E> {
        self.cs.set_low();
        self.spi.write(&[com.addr()])?;
        self.cs.set_high();
        Ok(())
    }

    fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        self.cs.set_low();

        let mut buffer = [reg.addr() | WRITE_SINGLE_BYTE, byte];
        self.spi.write(&mut buffer)?;

        self.cs.set_high();

        Ok(())
    }

    fn write_burst(&mut self, com: Command, buf: &mut [u8]) -> Result<(), E> {
        self.cs.set_low();

        // Hopefully the same as writing an array that starts with the command followed by buf
        self.spi.write(&[com.addr() | WRITE_BURST])?;
        self.spi.write(&buf)?;

        self.cs.set_high();

        Ok(())
    }

    fn modify_register<F>(&mut self, reg: Register, f: F) -> Result<(), E>
    where
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_register(reg)?;
        self.write_register(reg, f(r))?;
        Ok(())
    }
}

// Read/Write Offsets
const WRITE_SINGLE_BYTE: u8 = 0x00;
const WRITE_BURST: u8 = 0x40;
const READ_SINGLE_BYTE: u8 = 0x80;
const READ_BURST: u8 = 0xC0;

impl Register {
    fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Register {
    /* STATUS REGISTER */
    PARTNUM = 0xF0,        // Part number
    VERSION = 0xF1,        // Current version number
    FREQEST = 0xF2,        // Frequency offset estimate
    LQI = 0xF3,            // Demodulator estimate for link quality
    RSSI = 0xF4,           // Received signal strength indication
    MARCSTATE = 0xF5,      // Control state machine state
    WORTIME1 = 0xF6,       // High byte of WOR timer
    WORTIME0 = 0xF7,       // Low byte of WOR timer
    PKTSTATUS = 0xF8,      // Current GDOx status and packet status
    VCO_VC_DAC = 0xF9,     // Current setting from PLL cal module
    TXBYTES = 0xFA,        // Underflow and # of bytes in TXFIFO
    RXBYTES = 0xFB,        // Overflow and # of bytes in RXFIFO
    RCCTRL1_STATUS = 0xFC, // Last RC Oscillator Calibration Result
    RCCTRL0_STATUS = 0xFD, // Last RC Oscillator Calibration Result

    /* CONFIG REGISTER */
    IOCFG2 = 0x00,   // GDO2 output pin configuration
    IOCFG1 = 0x01,   // GDO1 output pin configuration
    IOCFG0 = 0x02,   // GDO0 output pin configuration
    FIFOTHR = 0x03,  // RX FIFO and TX FIFO thresholds
    SYNC1 = 0x04,    // Sync word, high byte
    SYNC0 = 0x05,    // Sync word, low byte
    PKTLEN = 0x06,   // Packet length
    PKTCTRL1 = 0x07, // Packet automation control
    PKTCTRL0 = 0x08, // Packet automation control
    ADDR = 0x09,     // Device address
    CHANNR = 0x0A,   // Channel number
    FSCTRL1 = 0x0B,  // Frequency synthesizer control
    FSCTRL0 = 0x0C,  // Frequency synthesizer control
    FREQ2 = 0x0D,    // Frequency control word, high byte
    FREQ1 = 0x0E,    // Frequency control word, middle byte
    FREQ0 = 0x0F,    // Frequency control word, low byte
    MDMCFG4 = 0x10,  // Modem configuration
    MDMCFG3 = 0x11,  // Modem configuration
    MDMCFG2 = 0x12,  // Modem configuration
    MDMCFG1 = 0x13,  // Modem configuration
    MDMCFG0 = 0x14,  // Modem configuration
    DEVIATN = 0x15,  // Modem deviation setting
    MCSM2 = 0x16,    // Main Radio Cntrl State Machine config
    MCSM1 = 0x17,    // Main Radio Cntrl State Machine config
    MCSM0 = 0x18,    // Main Radio Cntrl State Machine config
    FOCCFG = 0x19,   // Frequency Offset Compensation config
    BSCFG = 0x1A,    // Bit Synchronization configuration
    AGCCTRL2 = 0x1B, // AGC control
    AGCCTRL1 = 0x1C, // AGC control
    AGCCTRL0 = 0x1D, // AGC control
    WOREVT1 = 0x1E,  // High byte Event 0 timeout
    WOREVT0 = 0x1F,  // Low byte Event 0 timeout
    WORCTRL = 0x20,  // Wake On Radio control
    FREND1 = 0x21,   // Front end RX configuration
    FREND0 = 0x22,   // Front end TX configuration
    FSCAL3 = 0x23,   // Frequency synthesizer calibration
    FSCAL2 = 0x24,   // Frequency synthesizer calibration
    FSCAL1 = 0x25,   // Frequency synthesizer calibration
    FSCAL0 = 0x26,   // Frequency synthesizer calibration
    RCCTRL1 = 0x27,  // RC oscillator configuration
    RCCTRL0 = 0x28,  // RC oscillator configuration
    FSTEST = 0x29,   // Frequency synthesizer cal control
    PTEST = 0x2A,    // Production test
    AGCTEST = 0x2B,  // AGC test
    TEST2 = 0x2C,    // Various test settings
    TEST1 = 0x2D,    // Various test settings
    TEST0 = 0x2E,    // Various test settings
    PATABLE = 0x3E,
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

    /* FIFO COMMANDS */
    WRITE_BURST = 0x40,
    READ_BURST = 0xC0,
    TXFIFO_BURST = 0x7F,        //write burst only
    TXFIFO_SINGLE_BYTE = 0x3F,  //write single only
    RXFIFO_BURST = 0xFF,        //read burst only
    RXFIFO_SINGLE_BYTE = 0xBF,  //read single only
    PATABLE_BURST = 0x7E,       //power control read/write
    PATABLE_SINGLE_BYTE = 0xFE, //power control read/write
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
