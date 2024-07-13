/// Number of preamble bytes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum NumPreamble {
    // 2 preamble bytes
    Two = 0,
    // 3 preamble bytes
    Three = 1,
    // 4 preamble bytes
    Four = 2,
    // 6 preamble bytes
    Six = 3,
    // 8 preamble bytes
    Eight = 4,
    // 12 preamble bytes
    Twelve = 5,
    // 16 preamble bytes
    Sixteen = 6,
    // 24 preamble bytes
    TwentyFour = 7,
}

impl From<NumPreamble> for u8 {
    fn from(value: NumPreamble) -> Self {
        value as Self
    }
}
