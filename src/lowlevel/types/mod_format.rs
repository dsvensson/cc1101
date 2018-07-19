#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ModFormat {
    MOD_2FSK = 0x00,
    MOD_GFSK = 0x01,
    MOD_ASK_OOK = 0x03,
    MOD_4FSK = 0x04,
    MOD_MSK = 0x07,
}

impl ModFormat {
    pub fn value(self) -> u8 {
        self as u8
    }
}
