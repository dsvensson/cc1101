#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum NumPreamble {
    N_2 = 0x00,
    N_3 = 0x01,
    N_4 = 0x02,
    N_6 = 0x03,
    N_8 = 0x04,
    N_12 = 0x05,
    N_16 = 0x06,
    N_24 = 0x07,
}

#[allow(dead_code)]
impl NumPreamble {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
