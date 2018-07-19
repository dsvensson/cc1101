use core::ops::Range;

pub trait OffsetSize {
    fn offset(self) -> u8;
    fn size(self) -> u8;
}

impl OffsetSize for u8 {
    fn offset(self) -> u8 {
        self
    }

    fn size(self) -> u8 {
        1
    }
}

impl OffsetSize for Range<u8> {
    fn offset(self) -> u8 {
        self.start
    }

    fn size(self) -> u8 {
        self.end - self.start
    }
}

#[derive(Clone, Copy)]
pub struct Mask;

#[derive(Clone, Copy)]
pub struct R;

#[derive(Clone, Copy)]
pub struct W;
