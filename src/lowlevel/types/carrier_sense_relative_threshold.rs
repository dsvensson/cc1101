/// Relative RSSI increase threshold for asserting carrier sense.
/// CC1101 datasheet p. 86

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CarrierSenseRelativeThreshold {
    /// Relative carrier sense threshold disabled.
    Disabled = 0,
    /// Assert on 6 dB RSSI increase.
    Db6 = 1,
    /// Assert on 10 dB RSSI increase.
    Db10 = 2,
    /// Assert on 14 dB RSSI increase.
    Db14 = 3,
}

impl From<CarrierSenseRelativeThreshold> for u8 {
    fn from(value: CarrierSenseRelativeThreshold) -> Self {
        value as Self
    }
}
