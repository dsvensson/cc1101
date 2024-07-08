/// Sync word qualifier mode configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SyncCheck {
    /// No preamble/sync.
    DISABLED = 0x00,
    /// 15/16 sync word bits detected.
    CHECK_15_16 = 0x01,
    /// 16/16 sync word bits detected.
    CHECK_16_16 = 0x02,
    /// 30/32 sync word bits detected.
    CHECK_30_32 = 0x03,
    /// No preamble/sync, carrier-sense above threshold.
    CHECK_0_0_CS = 0x04,
    /// 15/16 + carrier-sense above threshold.
    CHECK_15_16_CS = 0x05,
    /// 16/16 + carrier-sense above threshold.
    CHECK_16_16_CS = 0x06,
    /// 30/32 + carrier-sense above threshold.
    CHECK_30_32_CS = 0x07,
}

impl From<SyncCheck> for u8 {
    fn from(value: SyncCheck) -> Self {
        value as Self
    }
}
