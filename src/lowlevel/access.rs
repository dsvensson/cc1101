#[allow(dead_code)]
pub enum Mode {
    Single = 0x00,
    Burst = 0x40,
}

impl Mode {
    pub fn offset(self, addr: u8) -> u8 {
        (self as u8) | addr
    }
}
