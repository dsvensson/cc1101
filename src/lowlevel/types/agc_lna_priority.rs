/// LNA/LNA2 gain reduction strategy for AGC.
/// CC1101 datasheet p. 86

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AgcLnaPriority {
    /// Decrease `LNA2` gain to minimum before decreasing `LNA` gain.
    Lna2First = 0,
    /// Decrease `LNA` gain before decreasing `LNA2` gain.
    LnaFirst = 1,
}

impl From<AgcLnaPriority> for u8 {
    fn from(value: AgcLnaPriority) -> Self {
        value as Self
    }
}
