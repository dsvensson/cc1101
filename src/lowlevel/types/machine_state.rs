/// Radio hardware machine states.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MachineState {
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

/// Machine State Error (Invalid State). Refer to the CC1101 datasheet: "10.3 SPI Read"
pub enum MachineStateError {
    InvalidState(u8),
}

impl From<MachineState> for u8 {
    fn from(value: MachineState) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for MachineState {
    type Error = MachineStateError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(MachineState::SLEEP),
            0x01 => Ok(MachineState::IDLE),
            0x02 => Ok(MachineState::XOFF),
            0x03 => Ok(MachineState::VCOON_MC),
            0x04 => Ok(MachineState::REGON_MC),
            0x05 => Ok(MachineState::MANCAL),
            0x06 => Ok(MachineState::VCOON),
            0x07 => Ok(MachineState::REGON),
            0x08 => Ok(MachineState::STARTCAL),
            0x09 => Ok(MachineState::BWBOOST),
            0x0A => Ok(MachineState::FS_LOCK),
            0x0B => Ok(MachineState::IFADCON),
            0x0C => Ok(MachineState::ENDCAL),
            0x0D => Ok(MachineState::RX),
            0x0E => Ok(MachineState::RX_END),
            0x0F => Ok(MachineState::RX_RST),
            0x10 => Ok(MachineState::TXRX_SWITCH),
            0x11 => Ok(MachineState::RXFIFO_OVERFLOW),
            0x12 => Ok(MachineState::FSTXON),
            0x13 => Ok(MachineState::TX),
            0x14 => Ok(MachineState::TX_END),
            0x15 => Ok(MachineState::RXTX_SWITCH),
            0x16 => Ok(MachineState::TXFIFO_UNDERFLOW),
            _ => Err(MachineStateError::InvalidState(value)),
        }
    }
}
