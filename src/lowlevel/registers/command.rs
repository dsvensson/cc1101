#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Command {
    /// Reset chip
    SRES = 0x30,
    /// Enable/calibrate freq synthesizer
    SFSTXON = 0x31,
    /// Turn off crystal oscillator.
    SXOFF = 0x32,
    /// Calibrate freq synthesizer & disable
    SCAL = 0x33,
    /// Enable RX.
    SRX = 0x34,
    /// Enable TX.
    STX = 0x35,
    /// Exit RX / TX
    SIDLE = 0x36,
    /// AFC adjustment of freq synthesizer
    SAFC = 0x37,
    /// Start automatic RX polling sequence
    SWOR = 0x38,
    /// Enter pwr down mode when CSn goes hi
    SPWD = 0x39,
    /// Flush the RX FIFO buffer.
    SFRX = 0x3A,
    /// Flush the TX FIFO buffer.
    SFTX = 0x3B,
    /// Reset real time clock.
    SWORRST = 0x3C,
    /// No operation.
    SNOP = 0x3D,
    /// Power Amplifier Table
    PATABLE = 0x3E,
    /// FIFO Access
    FIFO = 0x3F,
}

impl Command {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

impl Into<::lowlevel::registers::Register> for Command {
    fn into(self) -> ::lowlevel::registers::Register {
        ::lowlevel::registers::Register::Command(self)
    }
}
