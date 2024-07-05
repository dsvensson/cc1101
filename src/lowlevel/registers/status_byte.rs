/// Indicates the current main state machine mode
#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    /// IDLE state (Also reported for some transitional states instead of SETTLING or CALIBRATE)
    #[default]
    IDLE = 0b000,
    /// Receive mode
    RX = 0b001,
    /// Transmit mode
    TX = 0b010,
    /// Fast TX ready
    FSTXON = 0b011,
    /// Frequency synthesizer calibration is running
    CALIBRATE = 0b100,
    /// PLL is settling
    SETTLING = 0b101,
    /// RX FIFO has overflowed. Read out any useful data, then flush the FIFO with SFRX
    RXFIFO_OVERFLOW = 0b110,
    /// TX FIFO has underflowed. Acknowledge with SFTX
    TXFIFO_UNDERFLOW = 0b111,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0b000 => State::IDLE,
            0b001 => State::RX,
            0b010 => State::TX,
            0b011 => State::FSTXON,
            0b100 => State::CALIBRATE,
            0b101 => State::SETTLING,
            0b110 => State::RXFIFO_OVERFLOW,
            0b111 => State::TXFIFO_UNDERFLOW,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

/// Table 23: Status Byte
#[derive(Default, Clone, Copy)]
pub struct StatusByte {
    pub chip_rdy: bool,
    pub state: State,
    pub fifo_bytes_available: u8,
}

impl From<u8> for StatusByte {
    fn from(value: u8) -> Self {
        let status_byte = STATUS_BYTE(value);
        StatusByte {
            chip_rdy: (status_byte.chip_rdyn() == 0),
            state: State::from(status_byte.state()),
            fifo_bytes_available: status_byte.fifo_bytes_available(),
        }
    }
}

register!(STATUS_BYTE, 0b1000_0000, u8, {
    #[doc = "Stays high until power and crystal have stabilized. Should always be low when using the SPI interface."]
    chip_rdyn @ 7,
    #[doc = "Indicates the current main state machine mode"]
    state @ 4..6,
    #[doc = "The number of bytes available in the RX FIFO or free bytes in the TX FIFO"]
    fifo_bytes_available @ 0..3,
});
