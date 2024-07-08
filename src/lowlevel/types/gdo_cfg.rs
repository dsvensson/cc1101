/// General Purpose Control Pin Configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GdoCfg {
    /// Associated to the RX FIFO: Asserts when RX FIFO is filled at or above the RX FIFO threshold. De-asserts when RX FIFO is drained below the same threshold.
    RX_FIFO_FILLED = 0x00,
    /// Associated to the RX FIFO: Asserts when RX FIFO is filled at or above the RX FIFO threshold or the end of packet is reached. De-asserts when the RX FIFO is empty.
    RX_FIFO_FILLED_END_OF_PKT = 0x01,
    /// Associated to the TX FIFO: Asserts when the TX FIFO is filled at or above the TX FIFO threshold. De-asserts when the TX FIFO is below the same threshold.
    TX_FIFO_FILLED = 0x02,
    /// Associated to the TX FIFO: Asserts when TX FIFO is full. De-asserts when the TX FIFO is drained below the TX FIFO threshold.
    TX_FIFO_FULL = 0x03,
    /// Asserts when the RX FIFO has overflowed. De-asserts when the FIFO has been flushed.
    RX_FIFO_OVERFLOW = 0x04,
    /// Asserts when the TX FIFO has underflowed. De-asserts when the FIFO is flushed.
    TX_FIFO_UNDERFLOW = 0x05,
    /// Asserts when sync word has been sent / received, and de-asserts at the end of the packet. In RX, the pin will also deassert when a packet is discarded due to address or maximum length filtering or when the radio enters RXFIFO_OVERFLOW state. In TX the pin will de-assert if the TX FIFO underflows.
    SYNC_WORD = 0x06,
    /// Asserts when a packet has been received with CRC OK. De-asserts when the first byte is read from the RX FIFO.
    CRC_OK = 0x07,
    /// Preamble Quality Reached. Asserts when the PQI is above the programmed PQT value. De-asserted when the chip reenters RX state (MARCSTATE=0x0D) or the PQI gets below the programmed PQT value.
    PQT_REACHED = 0x08,
    /// Clear channel assessment. High when RSSI level is below threshold (dependent on the current CCA_MODE setting).
    CHANNEL_CLEAR = 0x09,
    /// Lock detector output. The PLL is in lock if the lock detector output has a positive transition or is constantly logic high. To check for PLL lock the lock detector output should be used as an interrupt for the MCU.
    PLL_LOCK = 0x0A,
    /// Serial Clock. Synchronous to the data in synchronous serial mode. In RX mode, data is set up on the falling edge by CC1101 when GDOx_INV=0. In TX mode, data is sampled by CC1101 on the rising edge of the serial clock when GDOx_INV=0.
    SERIAL_CLOCK = 0x0B,
    /// Serial Synchronous Data Output. Used for synchronous serial mode.
    SERIAL_SYNC_DATA_OUT = 0x0C,
    /// Serial Data Output. Used for asynchronous serial mode.
    SERIAL_DATA_OUT = 0x0D,
    /// Carrier sense. High if RSSI level is above threshold. Cleared when entering IDLE mode.
    CARRIER_SENSE = 0x0E,
    /// CRC_OK. The last CRC comparison matched. Cleared when entering/restarting RX mode.
    LAST_CRC_OK = 0x0F,

    /// RX_HARD_DATA\[1\]. Can be used together with RX_SYMBOL_TICK for alternative serial RX output.
    RX_HARD_DATA_1 = 0x16,
    /// RX_HARD_DATA\[0\]. Can be used together with RX_SYMBOL_TICK for alternative serial RX output.
    RX_HARD_DATA_0 = 0x17,

    /// PA_PD. Note: PA_PD will have the same signal level in SLEEP and TX states. To control an external PA or RX/TX switch in applications where the SLEEP state is used it is recommended to use GDOx_CFGx=0x2F instead.
    PA_PD = 0x1B,
    /// LNA_PD. Note: LNA_PD will have the same signal level in SLEEP and RX states. To control an external LNA or RX/TX switch in applications where the SLEEP state is used it is recommended to use GDOx_CFGx=0x2F instead.
    LNA_PD = 0x1C,
    /// RX_SYMBOL_TICK. Can be used together with RX_HARD_DATA for alternative serial RX output.
    RX_SYMBOL_TICK = 0x1D,

    /// WOR_EVNT0.
    WOR_EVNT0 = 0x24,
    /// WOR_EVNT1.
    WOR_EVNT1 = 0x25,
    /// CLK_256.
    CLK_256 = 0x26,
    /// CLK_32k.
    CLK_32k = 0x27,

    /// CHIP_RDYn.
    CHIP_RDYn = 0x29,

    /// XOSC_STABLE.
    XOSC_STABLE = 0x2B,

    /// High impedance (3-state).
    HIGH_IMPEDANCE = 0x2E,
    /// HW to 0 (HW1 achieved by setting GDOx_INV=1). Can be used to control an external LNA/PA or RX/TX switch.
    HARDWIRE_TO_0 = 0x2F,
    /// CLK_XOSC/1.
    CLK_XOSC_1 = 0x30,
    /// CLK_XOSC/1.5.
    CLK_XOSC_1_5 = 0x31,
    /// CLK_XOSC/2.
    CLK_XOSC_2 = 0x32,
    /// CLK_XOSC/3.
    CLK_XOSC_3 = 0x33,
    /// CLK_XOSC/4.
    CLK_XOSC_4 = 0x34,
    /// CLK_XOSC/6.
    CLK_XOSC_6 = 0x35,
    /// CLK_XOSC/8.
    CLK_XOSC_8 = 0x36,
    /// CLK_XOSC/12.
    CLK_XOSC_12 = 0x37,
    /// CLK_XOSC/16.
    CLK_XOSC_16 = 0x38,
    /// CLK_XOSC/24.
    CLK_XOSC_24 = 0x39,
    /// CLK_XOSC/32.
    CLK_XOSC_32 = 0x3A,
    /// CLK_XOSC/48.
    CLK_XOSC_48 = 0x3B,
    /// CLK_XOSC/64.
    CLK_XOSC_64 = 0x3C,
    /// CLK_XOSC/96.
    CLK_XOSC_96 = 0x3D,
    /// CLK_XOSC/128.
    CLK_XOSC_128 = 0x3E,
    /// CLK_XOSC/192.
    CLK_XOSC_192 = 0x3F,
}

impl From<GdoCfg> for u8 {
    fn from(value: GdoCfg) -> Self {
        value as Self
    }
}
