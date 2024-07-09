/// Modulation format configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Modulation {
    /// 2-FSK.
    BinaryFrequencyShiftKeying,
    /// GFSK.
    GaussianFrequencyShiftKeying,
    /// ASK / OOK.
    OnOffKeying,
    /// 4-FSK.
    FourFrequencyShiftKeying,
    /// MSK.
    MinimumShiftKeying,
}

/// Packet length configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PacketLength {
    /// Set packet length to a fixed value.
    Fixed(u8),
    /// Set upper bound of variable packet length.
    Variable(u8),
    /// Infinite packet length, streaming mode.
    Infinite,
}

/// Address check configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressFilter {
    /// No address check.
    Disabled,
    /// Address check, no broadcast.
    Device(u8),
    /// Address check and 0 (0x00) broadcast.
    DeviceLowBroadcast(u8),
    /// Address check and 0 (0x00) and 255 (0xFF) broadcast.
    DeviceHighLowBroadcast(u8),
}

/// Radio operational mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RadioMode {
    Idle,
    Sleep,
    Calibrate,
    Transmit,
    Receive,
}

/// Sync word configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyncMode {
    /// No sync word.
    Disabled,
    /// Match 15 of 16 bits of given sync word.
    MatchPartial(u16),
    /// Match 30 of 32 bits of a repetition of given sync word.
    MatchPartialRepeated(u16),
    /// Match 16 of 16 bits of given sync word.
    MatchFull(u16),
}

/// Command Strobes.
pub enum CommandStrobe {
    /// SRES
    ResetChip,
    /// SFSTXON
    EnableAndCalFreqSynth,
    /// SXOFF
    TurnOffXosc,
    /// SCAL
    CalFreqSynthAndTurnOff,
    /// SRX
    EnableRx,
    /// STX
    EnableTx,
    /// SIDLE
    ExitRxTx,
    /// SWOR
    StartWakeOnRadio,
    /// SPWD
    EnterPowerDownMode,
    /// SFRX
    FlushRxFifoBuffer,
    /// SFTX
    FlushTxFifoBuffer,
    /// SWORRST
    ResetRtcToEvent1,
    /// SNOP
    NoOperation,
}

/// Target amplitude from channel filter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TargetAmplitude {
    /// 24 dB
    Db24 = 0,
    /// 27 dB
    Db27 = 1,
    /// 30 dB
    Db30 = 2,
    /// 33 dB
    Db33 = 3,
    /// 36 dB
    Db36 = 4,
    /// 38 dB
    Db38 = 5,
    /// 40 dB
    Db40 = 6,
    /// 42 dB
    Db42 = 7,
}

impl From<TargetAmplitude> for u8 {
    fn from(value: TargetAmplitude) -> Self {
        value as Self
    }
}

/// Channel filter samples or OOK/ASK decision boundary for AGC.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum FilterLength {
    /// 8 filter samples for FSK/MSK, or 4 dB for OOK/ASK.
    Samples8 = 0,
    /// 16 filter samples for FSK/MSK, or 8 dB for OOK/ASK.
    Samples16 = 1,
    /// 32 filter samples for FSK/MSK, or 12 dB for OOK/ASK.
    Samples32 = 2,
    /// 64 filter samples for FSK/MSK, or 16 dB for OOK/ASK.
    Samples64 = 3,
}

impl From<FilterLength> for u8 {
    fn from(value: FilterLength) -> Self {
        value as Self
    }
}
