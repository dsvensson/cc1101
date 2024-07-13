/// Modulation format.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ModulationFormat {
    /// 2-FSK.
    BinaryFrequencyShiftKeying = 0,
    /// GFSK.
    GaussianFrequencyShiftKeying = 1,
    /// ASK / OOK.
    AmplitudeShiftOnOffKeying = 3,
    /// 4-FSK.
    QuaternaryFrequencyShiftKeying = 4,
    /// MSK.
    MinimumShiftKeying = 7,
}

impl From<ModulationFormat> for u8 {
    fn from(value: ModulationFormat) -> Self {
        value as Self
    }
}
