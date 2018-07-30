#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum SyncCheck {
    DISABLED = 0x00,
    CHECK_15_16 = 0x01,
    CHECK_16_16 = 0x02,
    CHECK_30_32 = 0x03,
    CHECK_0_0_CS = 0x04,
    CHECK_15_16_CS = 0x05,
    CHECK_16_16_CS = 0x06,
    CHECK_30_32_CS = 0x07,
}

impl SyncCheck {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
