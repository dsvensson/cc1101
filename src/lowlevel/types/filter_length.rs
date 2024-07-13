/// Filter length for frequency and amplitude modulations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FilterLength {
    FrequencyModulation(ChannelFilterSamples),
    AmplitudeModulation(DecisionBoundary),
}

impl From<FilterLength> for u8 {
    fn from(value: FilterLength) -> Self {
        match value {
            FilterLength::FrequencyModulation(inner) => inner.into(),
            FilterLength::AmplitudeModulation(inner) => inner.into(),
        }
    }
}

/// Channel filter samples.
/// 2-FSK, 4-FSK, MSK: Sets the averaging length for the amplitude from the channel filter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ChannelFilterSamples {
    /// 8 samples
    Samples8 = 0,
    /// 16 samples
    Samples16 = 1,
    /// 32 samples
    Samples32 = 2,
    /// 64 samples
    Samples64 = 3,
}

impl From<ChannelFilterSamples> for u8 {
    fn from(value: ChannelFilterSamples) -> Self {
        value as Self
    }
}

/// OOK/ASK decision boundary.
/// ASK, OOK: Sets the OOK/ASK decision boundary for OOK/ASK reception.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DecisionBoundary {
    /// 4 dB
    Db4 = 0,
    /// 8 dB
    Db8 = 1,
    /// 12 dB
    Db12 = 2,
    /// 16 dB
    Db16 = 3,
}

impl From<DecisionBoundary> for u8 {
    fn from(value: DecisionBoundary) -> Self {
        value as Self
    }
}
