#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum AddressCheck {
    DISABLED = 0x00,
    SELF = 0x01,
    SELF_LOW_BROADCAST = 0x02,
    SELF_HIGH_LOW_BROADCAST = 0x03,
}

impl AddressCheck {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
