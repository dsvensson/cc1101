#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum FifoThreshold {
    TX_61_RX_4 = 0x00,
    TX_57_RX_8 = 0x01,
    TX_53_RX_12 = 0x02,
    TX_49_RX_16 = 0x03,
    TX_45_RX_20 = 0x04,
    TX_41_RX_24 = 0x05,
    TX_37_RX_28 = 0x06,
    TX_33_RX_32 = 0x07,
    TX_29_RX_36 = 0x08,
    TX_25_RX_40 = 0x09,
    TX_21_RX_44 = 0x0A,
    TX_17_RX_48 = 0x0B,
    TX_13_RX_52 = 0x0C,
    TX_9_RX_56 = 0x0D,
    TX_5_RX_60 = 0x0E,
    TX_1_RX_64 = 0x0F,
}

impl FifoThreshold {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
