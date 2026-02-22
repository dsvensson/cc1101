/// Maximum allowable DVGA gain.
/// CC1101 datasheet p. 85

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MaxDvgaGain {
    /// All DVGA gain settings enabled
    AllEnabled = 0,
    /// The highest DVGA gain setting disabled
    HighestDisabled = 1,
    /// The 2 highest DVGA gain settings disabled
    Highest2Disabled = 2,
    /// The 3 highest DVGA gain settings disabled
    Highest3Disabled = 3,
}

impl From<MaxDvgaGain> for u8 {
    fn from(value: MaxDvgaGain) -> Self {
        value as Self
    }
}
