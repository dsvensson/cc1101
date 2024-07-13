/// Radio operational mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RadioMode {
    Idle,
    Sleep,
    Calibrate,
    Transmit,
    Receive,
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
