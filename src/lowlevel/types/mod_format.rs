/// Modulation format configuration.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ModFormat {
    /// 2-FSK.
    MOD_2FSK = 0x00,
    /// GFSK.
    MOD_GFSK = 0x01,
    /// ASK / OOK.
    MOD_ASK_OOK = 0x03,
    /// 4-FSK.
    MOD_4FSK = 0x04,
    /// MSK.
    MOD_MSK = 0x07,
}

impl ModFormat {
    pub fn value(self) -> u8 {
        self as u8
    }
}
