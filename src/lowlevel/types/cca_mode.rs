/// Clear Channel Assessment Mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CcaMode {
    /// Clear channel indication: Always
    CciAlways = 0,
    /// Clear channel indication: If RSSI below threshold
    CciRssiBelowThreshold = 1,
    /// Clear channel indication: Unless currently receiving a packet
    CciUnlessCurrentlyReceivingPacket = 2,
    /// Clear channel indication: If RSSI below threshold unless currently receiving a packet
    CciRssiBelowThresholdUnlessCurrentlyReceivingPacket = 3,
}

impl From<CcaMode> for u8 {
    fn from(value: CcaMode) -> Self {
        value as Self
    }
}
