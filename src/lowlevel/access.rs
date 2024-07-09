#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    Single = 0x00,
    Burst = 0x40,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Access {
    Read = 0x80,
    Write = 0x00,
}
