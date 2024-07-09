/// TX FIFO and RX FIFO threshold configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FifoThreshold {
    /// 61 bytes in TX, 4 bytes in RX.
    TX_61_RX_4 = 0x00,
    /// 57 bytes in TX, 8 bytes in RX.
    TX_57_RX_8 = 0x01,
    /// 53 bytes in TX, 12 bytes in RX.
    TX_53_RX_12 = 0x02,
    /// 49 bytes in TX, 16 bytes in RX.
    TX_49_RX_16 = 0x03,
    /// 45 bytes in TX, 20 bytes in RX.
    TX_45_RX_20 = 0x04,
    /// 41 bytes in TX, 24 bytes in RX.
    TX_41_RX_24 = 0x05,
    /// 37 bytes in TX, 28 bytes in RX.
    TX_37_RX_28 = 0x06,
    /// 33 bytes in TX, 32 bytes in RX.
    TX_33_RX_32 = 0x07,
    /// 29 bytes in TX, 36 bytes in RX.
    TX_29_RX_36 = 0x08,
    /// 25 bytes in TX, 40 bytes in RX.
    TX_25_RX_40 = 0x09,
    /// 21 bytes in TX, 44 bytes in RX.
    TX_21_RX_44 = 0x0A,
    /// 17 bytes in TX, 48 bytes in RX.
    TX_17_RX_48 = 0x0B,
    /// 13 bytes in TX, 52 bytes in RX.
    TX_13_RX_52 = 0x0C,
    /// 9 bytes in TX, 56 bytes in RX.
    TX_9_RX_56 = 0x0D,
    /// 5 bytes in TX, 60 bytes in RX.
    TX_5_RX_60 = 0x0E,
    /// 1 byte in TX, 64 bytes in RX.
    TX_1_RX_64 = 0x0F,
}

impl From<FifoThreshold> for u8 {
    fn from(value: FifoThreshold) -> Self {
        value as Self
    }
}
