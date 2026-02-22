/// Maximum allowable LNA + LNA2 gain.
/// CC1101 datasheet p. 85

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MaxLnaGain {
    /// Maximum possible LNA gain
    Max = 0,
    /// 2.6 dB below
    BelowMax2_6 = 1,
    /// 6.1 dB below
    BelowMax6_1 = 2,
    /// 7.4 dB below
    BelowMax7_4 = 3,
    /// 9.2 dB below
    BelowMax9_2 = 4,
    /// 11.5 dB below
    BelowMax11_5 = 5,
    /// 14.6 dB below
    BelowMax14_6 = 6,
    /// 17.1 dB below
    BelowMax17_1 = 7,
}

impl From<MaxLnaGain> for u8 {
    fn from(value: MaxLnaGain) -> Self {
        value as Self
    }
}
