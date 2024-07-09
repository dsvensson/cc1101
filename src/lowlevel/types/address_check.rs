/// Address check configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AddressCheck {
    /// No address check.
    DISABLED = 0x00,
    /// Address check, no broadcast.
    SELF = 0x01,
    /// Address check and 0 (0x00) broadcast
    SELF_LOW_BROADCAST = 0x02,
    /// Address check and 0 (0x00) and 255 (0xFF) broadcast.
    SELF_HIGH_LOW_BROADCAST = 0x03,
}

impl From<AddressCheck> for u8 {
    fn from(value: AddressCheck) -> Self {
        value as Self
    }
}
