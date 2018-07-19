#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum LengthConfig {
    FIXED = 0x00,
    VARIABLE = 0x01,
    INFINITE = 0x02,
}

impl LengthConfig {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
