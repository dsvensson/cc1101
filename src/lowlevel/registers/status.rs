#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Status {
    /// Chip part number
    PARTNUM = 0x30,
    /// Chip version number
    VERSION = 0x31,
    /// Frequency Offset Estimate from Demodulator
    FREQEST = 0x32,
    /// Demodulator Estimate for Link Quality
    LQI = 0x33,
    /// Received Signal Strength Indication
    RSSI = 0x34,
    /// Main Radio Control State Machine State
    MARCSTATE = 0x35,
    /// High Byte of WOR Time
    WORTIME1 = 0x36,
    /// Low Byte of WOR Time
    WORTIME0 = 0x37,
    /// Current GDOx Status and Packet Status
    PKTSTATUS = 0x38,
    /// Current Setting from PLL Calibration Module
    VCO_VC_DAC = 0x39,
    /// Underflow and Number of Bytes
    TXBYTES = 0x3A,
    /// Overflow and Number of Bytes
    RXBYTES = 0x3B,
    /// Last RC Oscillator Calibration Result
    RCCTRL1_STATUS = 0x3C,
    /// Last RC Oscillator Calibration Result
    RCCTRL0_STATUS = 0x3D,
}

impl Status {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

impl From<Status> for crate::lowlevel::registers::Register {
    fn from(val: Status) -> Self {
        crate::lowlevel::registers::Register::Status(val)
    }
}

register!(PARTNUM, 0b0000_0000, u8, {
    #[doc = "Chip part number"]
    partnum @ 0..7,
});

register!(VERSION, 0b0001_0100, u8, {
    #[doc = "Chip version number"]
    version @ 0..7,
});

register!(FREQEST, 0b0000_0000, u8, {
    #[doc = "The estimated frequency offset (2’s complement) of the carrier"]
    freqoff_est @ 0..7,
});

register!(LQI, 0b0000_0000, u8, {
    #[doc = "The last CRC comparison matched."]
    crc_ok @ 7,
    #[doc = "The Link Quality Indicator estimates how easily a received signal can be demodulated"]
    lqi @ 0..6,
});

register!(RSSI, 0b0000_0000, u8, {
    #[doc = "Received signal strength indicator"]
    rssi @ 0..7,
});

register!(MARCSTATE, 0b0000_0000, u8, {
    #[doc = "Main Radio Control FSM State"]
    marc_state @ 0..4,
});

register!(WORTIME1, 0b0000_0000, u8, {
    #[doc = "High byte of timer value in WOR module"]
    time @ 0..7,
});

register!(WORTIME0, 0b0000_0000, u8, {
    #[doc = "Low byte of timer value in WOR module"]
    time @ 0..7,
});

register!(PKTSTATUS, 0b0000_0000, u8, {
    #[doc = "The last CRC comparison matched"]
    crc_ok @ 7,
    #[doc = "Carrier sense"]
    cs @ 6,
    #[doc = "Preamble Quality reached"]
    pqt_reached @ 5,
    #[doc = "Channel is clear"]
    cca @ 4,
    #[doc = "Start of Frame Delimiter"]
    sfd @ 3,
    #[doc = "Current GDO2 value"]
    gdo2 @ 2,
    #[doc = "Current GDO0 value"]
    gdo0 @ 0,
});

register!(VCO_VC_DAC, 0b0000_0000, u8, {
    #[doc = "Status register for test only"]
    vco_vc_dac @ 0..7,
});

register!(TXBYTES, 0b0000_0000, u8, {
    #[doc = "TX FIFO underflow"]
    txfifo_underflow @ 7,
    #[doc = "Number of bytes in TX FIFO"]
    num_txbytes @ 0..6,
});

register!(RXBYTES, 0b0000_0000, u8, {
    #[doc = "RX FIFO overflow"]
    rxfifo_overflow @ 7,
    #[doc = "Number of bytes in RX FIFO"]
    num_rxbytes @ 0..6,
});

register!(RCCTRL1_STATUS, 0b0000_0000, u8, {
    #[doc = "Contains the value from the last run of the RC oscillator calibration routine"]
    rcctrl1_status @ 0..6,
});

register!(RCCTRL0_STATUS, 0b0000_0000, u8, {
    #[doc = "Contains the value from the last run of the RC oscillator calibration routine"]
    rcctrl0_status @ 0..6,
});
