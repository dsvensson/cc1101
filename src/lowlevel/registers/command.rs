#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// Reset chip.
    SRES = 0x30,
    /// Enable and calibrate frequency synthesizer (if MCSM0.FS_AUTOCAL=1). If in RX (with CCA): Go to a wait state where only the synthesizer is running (for quick RX / TX turnaround).
    SFSTXON = 0x31,
    /// Turn off crystal oscillator.
    SXOFF = 0x32,
    /// Calibrate frequency synthesizer and turn it off. SCAL can be strobed from IDLE mode without setting manual calibration mode (MCSM0.FS_AUTOCAL=0)
    SCAL = 0x33,
    /// Enable RX. Perform calibration first if coming from IDLE and MCSM0.FS_AUTOCAL=1.
    SRX = 0x34,
    /// In IDLE state: Enable TX. Perform calibration first if MCSM0.FS_AUTOCAL=1. If in RX state and CCA is enabled: Only go to TX if channel is clear.
    STX = 0x35,
    /// Exit RX / TX, turn off frequency synthesizer and exit Wake-On-Radio mode if applicable.
    SIDLE = 0x36,
    /// Start automatic RX polling sequence (Wake-on-Radio) as described in Section 19.5 if WORCTRL.RC_PD=0.
    SWOR = 0x38,
    /// Enter power down mode when CSn goes high.
    SPWD = 0x39,
    /// Flush the RX FIFO buffer. Only issue SFRX in IDLE or RXFIFO_OVERFLOW states.
    SFRX = 0x3A,
    /// Flush the TX FIFO buffer. Only issue SFTX in IDLE or TXFIFO_UNDERFLOW states.
    SFTX = 0x3B,
    /// Reset real time clock to Event1 value.
    SWORRST = 0x3C,
    /// No operation. May be used to get access to the chip status byte.
    SNOP = 0x3D,
}

impl Command {
    pub fn addr(
        &self,
        access: crate::lowlevel::access::Access,
        mode: crate::lowlevel::access::Mode,
    ) -> u8 {
        (access as u8) | (mode as u8) | (*self as u8)
    }
}

impl From<Command> for crate::lowlevel::registers::Register {
    fn from(value: Command) -> Self {
        crate::lowlevel::registers::Register::Command(value)
    }
}
