//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::{Transfer, Write};
//use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;

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

    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        self.cs.set_low();

        let mut buffer = [reg.addr() | READ_SINGLE_BYTE, 0];
        self.spi.transfer(&mut buffer)?;

        self.cs.set_high();

        Ok(buffer[1])
    }

    fn read_burst(&mut self, com: Command, mut buf: &mut [u8]) -> Result<(), E> {
        self.cs.set_low();
        /*
        let mut rx_buf: [u8; buf.len()] = [0; buf.len()+1];
        rx_buf[0] = com.addr() | READ_BURST;

        self.spi.transfer(&mut rx_buf)?;

        for i in 1..buf.len()+1 {
            buf[i-1] = rx_buf[i];
        }
        */

        // Hopefully the same as transferring an array that starts with the command followed by buf
        self.spi.write(&[com.addr() | READ_BURST]);
        self.spi.transfer(&mut buf)?;

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
        self.spi.write(&[com.addr() | WRITE_BURST]);
        self.spi.write(&buf)?;

        self.cs.set_high();

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
    PARTNUM         = 0xF0,   // Part number
    VERSION         = 0xF1,   // Current version number
    FREQEST         = 0xF2,   // Frequency offset estimate
    LQI             = 0xF3,   // Demodulator estimate for link quality
    RSSI            = 0xF4,   // Received signal strength indication
    MARCSTATE       = 0xF5,   // Control state machine state
    WORTIME1        = 0xF6,   // High byte of WOR timer
    WORTIME0        = 0xF7,   // Low byte of WOR timer
    PKTSTATUS       = 0xF8,   // Current GDOx status and packet status
    VCO_VC_DAC      = 0xF9,   // Current setting from PLL cal module
    TXBYTES         = 0xFA,   // Underflow and # of bytes in TXFIFO
    RXBYTES         = 0xFB,   // Overflow and # of bytes in RXFIFO
    RCCTRL1_STATUS  = 0xFC,   // Last RC Oscillator Calibration Result
    RCCTRL0_STATUS  = 0xFD,   // Last RC Oscillator Calibration Result

    /* CONFIG REGISTER */
    IOCFG2    = 0x00,         // GDO2 output pin configuration
    IOCFG1    = 0x01,         // GDO1 output pin configuration
    IOCFG0    = 0x02,         // GDO0 output pin configuration
    FIFOTHR   = 0x03,         // RX FIFO and TX FIFO thresholds
    SYNC1     = 0x04,         // Sync word, high byte
    SYNC0     = 0x05,         // Sync word, low byte
    PKTLEN    = 0x06,         // Packet length
    PKTCTRL1  = 0x07,         // Packet automation control
    PKTCTRL0  = 0x08,         // Packet automation control
    ADDR      = 0x09,         // Device address
    CHANNR    = 0x0A,         // Channel number
    FSCTRL1   = 0x0B,         // Frequency synthesizer control
    FSCTRL0   = 0x0C,         // Frequency synthesizer control
    FREQ2     = 0x0D,         // Frequency control word, high byte
    FREQ1     = 0x0E,         // Frequency control word, middle byte
    FREQ0     = 0x0F,         // Frequency control word, low byte
    MDMCFG4   = 0x10,         // Modem configuration
    MDMCFG3   = 0x11,         // Modem configuration
    MDMCFG2   = 0x12,         // Modem configuration
    MDMCFG1   = 0x13,         // Modem configuration
    MDMCFG0   = 0x14,         // Modem configuration
    DEVIATN   = 0x15,         // Modem deviation setting
    MCSM2     = 0x16,         // Main Radio Cntrl State Machine config
    MCSM1     = 0x17,         // Main Radio Cntrl State Machine config
    MCSM0     = 0x18,         // Main Radio Cntrl State Machine config
    FOCCFG    = 0x19,         // Frequency Offset Compensation config
    BSCFG     = 0x1A,         // Bit Synchronization configuration
    AGCCTRL2  = 0x1B,         // AGC control
    AGCCTRL1  = 0x1C,         // AGC control
    AGCCTRL0  = 0x1D,         // AGC control
    WOREVT1   = 0x1E,         // High byte Event 0 timeout
    WOREVT0   = 0x1F,         // Low byte Event 0 timeout
    WORCTRL   = 0x20,         // Wake On Radio control
    FREND1    = 0x21,         // Front end RX configuration
    FREND0    = 0x22,         // Front end TX configuration
    FSCAL3    = 0x23,         // Frequency synthesizer calibration
    FSCAL2    = 0x24,         // Frequency synthesizer calibration
    FSCAL1    = 0x25,         // Frequency synthesizer calibration
    FSCAL0    = 0x26,         // Frequency synthesizer calibration
    RCCTRL1   = 0x27,         // RC oscillator configuration
    RCCTRL0   = 0x28,         // RC oscillator configuration
    FSTEST    = 0x29,         // Frequency synthesizer cal control
    PTEST     = 0x2A,         // Production test
    AGCTEST   = 0x2B,         // AGC test
    TEST2     = 0x2C,         // Various test settings
    TEST1     = 0x2D,         // Various test settings
    TEST0     = 0x2E,         // Various test settings
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
    SRES      = 0x30,         // Reset chip
    SFSTXON   = 0x31,         // Enable/calibrate freq synthesizer
    SXOFF     = 0x32,         // Turn off crystal oscillator.
    SCAL      = 0x33,         // Calibrate freq synthesizer & disable
    SRX       = 0x34,         // Enable RX.
    STX       = 0x35,         // Enable TX.
    SIDLE     = 0x36,         // Exit RX / TX
    SAFC      = 0x37,         // AFC adjustment of freq synthesizer
    SWOR      = 0x38,         // Start automatic RX polling sequence
    SPWD      = 0x39,         // Enter pwr down mode when CSn goes hi
    SFRX      = 0x3A,         // Flush the RX FIFO buffer.
    SFTX      = 0x3B,         // Flush the TX FIFO buffer.
    SWORRST   = 0x3C,         // Reset real time clock.
    SNOP      = 0x3D,         // No operation.

    /* FIFO COMMANDS */
    WRITE_BURST          = 0x40,
    READ_BURST           = 0xC0,
    TXFIFO_BURST         = 0x7F,   //write burst only
    TXFIFO_SINGLE_BYTE   = 0x3F,   //write single only
    RXFIFO_BURST         = 0xFF,   //read burst only
    RXFIFO_SINGLE_BYTE   = 0xBF,   //read single only
    PATABLE_BURST        = 0x7E,   //power control read/write
    PATABLE_SINGLE_BYTE  = 0xFE,   //power control read/write
}