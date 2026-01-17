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
    /// No sync word, no carrier sense.
    Disabled,
    /// Require 15 of 16 sync bits to match.
    Match15of16(u16),
    /// Require all 16 sync bits to match.
    Match16of16(u16),
    /// Require 30 of 32 sync bits (two repeated sync words).
    Match30of32(u16),
    /// No sync, but require carrier sense above threshold.
    CarrierSenseOnly,
    /// 15 of 16 sync bits + carrier sense.
    Match15of16Cs(u16),
    /// 16 of 16 sync bits + carrier sense.
    Match16of16Cs(u16),
    /// 30 of 32 sync bits + carrier sense.
    Match30of32Cs(u16),
}
