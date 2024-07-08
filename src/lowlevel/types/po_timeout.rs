/// Programs the number of times the six-bit ripple counter must expire after XOSC has stabilized before CHP_RDYn goes low.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PoTimeout {
    /// Approx. 2.3 – 2.4 μs.
    EXPIRE_COUNT_1 = 0x00,
    /// Approx. 37 – 39 μs.
    EXPIRE_COUNT_16 = 0x01,
    /// Approx. 149 – 155 μs.
    EXPIRE_COUNT_64 = 0x02,
    /// Approx. 597 – 620 μs.
    EXPIRE_COUNT_256 = 0x03,
}

impl From<PoTimeout> for u8 {
    fn from(value: PoTimeout) -> Self {
        value as Self
    }
}
