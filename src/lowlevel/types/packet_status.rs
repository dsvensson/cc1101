use crate::lowlevel::traits::R;
use crate::PKTSTATUS;

/// Structure representing readout of PKTSTATUS register
#[derive(Clone, Copy, Debug)]
pub struct PacketStatus {
    pub crc_ok: bool,
    pub carrier_sense: bool,
    pub preamble_quality_reached: bool,
    pub cca_channel_clear: bool,
    pub sof_delimiter: bool,
    pub gdo2: bool,
    pub gdo0: bool,
}

pub struct BoolU8(u8);

impl From<BoolU8> for bool {
    fn from(value: BoolU8) -> Self {
        match value.0 {
            0 => false,
            1 => true,
            _ => panic!("Invalid value: u8 must be 0 or 1"),
        }
    }
}

impl From<PKTSTATUS<R>> for PacketStatus {
    fn from(value: PKTSTATUS<R>) -> Self {
        Self {
            crc_ok: BoolU8(value.crc_ok()).into(),
            carrier_sense: BoolU8(value.cs()).into(),
            preamble_quality_reached: BoolU8(value.pqt_reached()).into(),
            cca_channel_clear: BoolU8(value.cca()).into(),
            sof_delimiter: BoolU8(value.sfd()).into(),
            gdo2: BoolU8(value.gdo2()).into(),
            gdo0: BoolU8(value.gdo0()).into(),
        }
    }
}
