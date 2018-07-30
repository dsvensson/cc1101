#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
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

impl MachineState {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
