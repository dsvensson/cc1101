#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum PoTimeout {
    EXPIRE_COUNT_1 = 0x00,
    EXPIRE_COUNT_16 = 0x01,
    EXPIRE_COUNT_64 = 0x02,
    EXPIRE_COUNT_256 = 0x03,
}

impl PoTimeout {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
