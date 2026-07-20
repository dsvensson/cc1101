/// Format of RX and TX data (`PKTCTRL0.PKT_FORMAT`).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PacketFormat {
    /// Normal mode, use FIFOs for RX and TX.
    Fifo = 0,
    /// Synchronous serial mode.
    SynchronousSerial = 1,
    /// Random TX mode; sends random data using a PN9 generator.
    RandomTx = 2,
    /// Asynchronous serial mode (raw data pass-through).
    AsynchronousSerial = 3,
}

impl From<PacketFormat> for u8 {
    fn from(value: PacketFormat) -> Self {
        value as u8
    }
}
