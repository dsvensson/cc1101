#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MultiByte {
    /// Power Amplifier Table
    PATABLE = 0x3E,
    /// FIFO Access
    FIFO = 0x3F,
}

impl MultiByte {
    pub fn addr(
        &self,
        access: crate::lowlevel::access::Access,
        mode: crate::lowlevel::access::Mode,
    ) -> u8 {
        (access as u8) | (mode as u8) | (*self as u8)
    }
}

impl From<MultiByte> for crate::lowlevel::registers::Register {
    fn from(value: MultiByte) -> Self {
        crate::lowlevel::registers::Register::MultiByte(value)
    }
}
