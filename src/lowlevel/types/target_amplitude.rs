/// Target amplitude from channel filter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TargetAmplitude {
    /// 24 dB
    Db24 = 0,
    /// 27 dB
    Db27 = 1,
    /// 30 dB
    Db30 = 2,
    /// 33 dB
    Db33 = 3,
    /// 36 dB
    Db36 = 4,
    /// 38 dB
    Db38 = 5,
    /// 40 dB
    Db40 = 6,
    /// 42 dB
    Db42 = 7,
}

impl From<TargetAmplitude> for u8 {
    fn from(value: TargetAmplitude) -> Self {
        value as Self
    }
}
