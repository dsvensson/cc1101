#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum AutoCalibration {
    DISABLED = 0x00,
    FROM_IDLE = 0x01,
    TO_IDLE = 0x02,
    TO_IDLE_EVERY_4TH = 0x03,
}

impl AutoCalibration {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
