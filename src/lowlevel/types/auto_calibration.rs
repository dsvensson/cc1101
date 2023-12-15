/// Configure what state transitions result in auto-calibration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AutoCalibration {
    /// Never (manually calibrate using SCAL strobe).
    Disabled = 0x00,
    /// When going from IDLE to RX or TX (or FSTXON).
    FromIdle = 0x01,
    /// When going from RX or TX back to IDLE automatically.
    ToIdle = 0x02,
    /// Every 4th time when going from RX or TX to IDLE automatically.
    ToIdleEvery4th = 0x03,
}

impl From<AutoCalibration> for u8 {
    fn from(value: AutoCalibration) -> u8 {
        value as u8
    }
}
