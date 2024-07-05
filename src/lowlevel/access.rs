#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Single = 0x00,
    Burst = 0x40,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Access {
    Read = 0x80,
    Write = 0x00,
}
