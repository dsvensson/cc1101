#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GdoCfg {
    RX_FIFO_FILLED = 0x00,
    RX_FIFO_FILLED_END_OF_PKT = 0x01,
    TX_FIFO_FILLED = 0x02,
    TX_FIFO_FULL = 0x03,
    RX_FIFO_OVERFLOW = 0x04,
    TX_FIFO_UNDERFLOW = 0x05,
    SYNC_WORD = 0x06,
    CRC_OK = 0x07,
    PQT_REACHED = 0x08,
    CHANNEL_CLEAR = 0x09,
    PLL_LOCK = 0x0A,
    SERIAL_CLOCK = 0x0B,
    SERIAL_SYNC_DATA_OUT = 0x0C,
    SERIAL_DATA_OUT = 0x0D,
    CARRIER_SENSE = 0x0E,
    LAST_CRC_OK = 0x0F,

    RX_HARD_DATA_1 = 0x16,
    RX_HARD_DATA_0 = 0x17,

    PA_PD = 0x1B,
    LNA_PD = 0x1C,
    RX_SYMBOL_TICK = 0x1D,

    WOR_EVNT0 = 0x24,
    WOR_EVNT1 = 0x25,
    CLK_256 = 0x26,
    CLK_32k = 0x27,

    CHIP_RDYn = 0x29,

    XOSC_STABLE = 0x2B,

    HIGH_IMPEDANCE = 0x2E,
    HARDWIRE_TO_0 = 0x2F,
    CLK_XOSC_1 = 0x30,
    CLK_XOSC_1_5 = 0x31,
    CLK_XOSC_2 = 0x32,
    CLK_XOSC_3 = 0x33,
    CLK_XOSC_4 = 0x34,
    CLK_XOSC_6 = 0x35,
    CLK_XOSC_8 = 0x36,
    CLK_XOSC_12 = 0x37,
    CLK_XOSC_16 = 0x38,
    CLK_XOSC_24 = 0x39,
    CLK_XOSC_32 = 0x3A,
    CLK_XOSC_48 = 0x3B,
    CLK_XOSC_64 = 0x3C,
    CLK_XOSC_96 = 0x3D,
    CLK_XOSC_128 = 0x3E,
    CLK_XOSC_192 = 0x3F,
}

#[allow(dead_code)]
impl GdoCfg {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
