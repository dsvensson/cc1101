/// Packet length configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LengthConfig {
    /// Fixed packet length mode. Length configured in PKTLEN register.
    FIXED = 0x00,
    /// Variable packet length mode. Packet length configured by the first byte after sync word.
    VARIABLE = 0x01,
    /// Infinite packet length mode.
    INFINITE = 0x02,
}

impl LengthConfig {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
