/// Configure what state transitions result in auto-calibration.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum AutoCalibration {
    /// Never (manually calibrate using SCAL strobe).
    DISABLED = 0x00,
    /// When going from IDLE to RX or TX (or FSTXON).
    FROM_IDLE = 0x01,
    /// When going from RX or TX back to IDLE automatically.
    TO_IDLE = 0x02,
    /// Every 4th time when going from RX or TX to IDLE automatically.
    TO_IDLE_EVERY_4TH = 0x03,
}

impl AutoCalibration {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
