use crate::lowlevel::access;

/// Declare a read-only bit-field view over a single byte that has no address or
/// access policy — e.g. the chip status byte returned on MISO during every SPI
/// access. Generates a `NAME(u8)` wrapper with a getter per field.
macro_rules! view {
    ($NAME:ident { $(#[$($fattr:tt)*] $field:ident @ $range:expr,)+ }) => {
        #[allow(non_camel_case_types)]
        struct $NAME(u8);
        impl $NAME {
            $(
                #[$($fattr)*]
                fn $field(&self) -> u8 {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};
                    let offset = $range.offset();
                    let mask = ((1u16 << $range.size().to_wider()) - 1) as u8;
                    (self.0 >> offset) & mask
                }
            )+
        }
    };
}

mod status_byte;
pub use self::status_byte::*;

/// Single-byte bit-field register that can be read (`Read` policy ≠ `Reject`).
pub trait Readable {
    /// Read view (exposes field getters).
    type View;
    const ADDR: u8;
    const MODE: access::Mode;
    fn view(bits: u8) -> Self::View;
}

/// Single-byte bit-field register that can be written / modified (`Write` ≠ `Reject`).
pub trait Writable {
    /// Write view (exposes field setters).
    type View;
    const ADDR: u8;
    const MODE: access::Mode;
    /// Reset value, used as the base for `write_register`.
    const RESET: u8;
    fn view(bits: u8) -> Self::View;
    fn bits(view: Self::View) -> u8;
}

/// Command strobe: address-only trigger (single access), returns the chip status.
pub trait Strobe {
    const ADDR: u8;
    /// Whether the strobe has no side effect (only `SNOP`). When true, the status
    /// byte returned during the strobe is a valid current read; otherwise it
    /// reflects the pre-transition state and should not be treated as current.
    const NO_EFFECT: bool;
}

/// Multi-byte burst region that can be read (FIFO RX, PATABLE).
pub trait BurstRead {
    const ADDR: u8;
}

/// Multi-byte burst region that can be written (FIFO TX, PATABLE).
pub trait BurstWrite {
    const ADDR: u8;
}

/// Declare a category of single-byte bit-field registers (e.g. config, status).
macro_rules! bitfields {
    ($cat:ident, read = $rd:ident, write = $wr:ident, {
        $(
            $(#[$($rmeta:tt)*])*
            $NAME:ident @ $addr:literal = $reset:literal {
                $(#[$($fattr:tt)*] $field:ident @ $range:expr,)+
            }
        )*
    }) => {
        $(
            $(#[$($rmeta)*])*
            #[allow(non_camel_case_types)]
            pub struct $NAME<MODE> {
                bits: u8,
                _mode: core::marker::PhantomData<MODE>,
            }
            bitfields!(@getters $NAME, $rd, { $(#[$($fattr)*] $field @ $range,)+ });
            bitfields!(@setters $NAME, $wr, { $(#[$($fattr)*] $field @ $range,)+ });
        )*
        pub mod $cat {
            $(
                $(#[$($rmeta)*])*
                #[allow(non_camel_case_types)]
                pub struct $NAME;
                bitfields!(@readable $NAME, $addr, $rd);
                bitfields!(@writable $NAME, $addr, $reset, $wr);
            )*
        }
    };

    // field getters on the read view (skipped when Read => Reject)
    (@getters $NAME:ident, Reject, { $($ignore:tt)* }) => {};
    (@getters $NAME:ident, $mode:ident, {
        $(#[$($fattr:tt)*] $field:ident @ $range:expr,)+
    }) => {
        impl $NAME<crate::lowlevel::traits::R> {
            $(
                #[$($fattr)*]
                pub fn $field(&self) -> u8 {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};
                    let offset = $range.offset();
                    let mask = ((1u16 << $range.size().to_wider()) - 1) as u8;
                    (self.bits >> offset) & mask
                }
            )+
        }
    };

    // field setters on the write view (skipped when Write => Reject)
    (@setters $NAME:ident, Reject, { $($ignore:tt)* }) => {};
    (@setters $NAME:ident, $mode:ident, {
        $(#[$($fattr:tt)*] $field:ident @ $range:expr,)+
    }) => {
        impl $NAME<crate::lowlevel::traits::W> {
            $(
                #[$($fattr)*]
                pub fn $field(mut self, value: u8) -> Self {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};
                    let offset = $range.offset();
                    let mask = ((1u16 << $range.size().to_wider()) - 1) as u8;
                    self.bits = (self.bits & !(mask << offset)) | ((value & mask) << offset);
                    self
                }
            )+
        }
    };

    (@readable $NAME:ident, $addr:literal, Reject) => {};
    (@readable $NAME:ident, $addr:literal, $mode:ident) => {
        impl crate::lowlevel::registers::Readable for $NAME {
            type View = super::$NAME<crate::lowlevel::traits::R>;
            const ADDR: u8 = $addr;
            const MODE: crate::lowlevel::access::Mode = bitfields!(@mode $mode);
            fn view(bits: u8) -> Self::View {
                super::$NAME { bits, _mode: core::marker::PhantomData }
            }
        }
    };

    (@writable $NAME:ident, $addr:literal, $reset:literal, Reject) => {};
    (@writable $NAME:ident, $addr:literal, $reset:literal, $mode:ident) => {
        impl crate::lowlevel::registers::Writable for $NAME {
            type View = super::$NAME<crate::lowlevel::traits::W>;
            const ADDR: u8 = $addr;
            const MODE: crate::lowlevel::access::Mode = bitfields!(@mode $mode);
            const RESET: u8 = $reset;
            fn view(bits: u8) -> Self::View {
                super::$NAME { bits, _mode: core::marker::PhantomData }
            }
            fn bits(view: Self::View) -> u8 {
                view.bits
            }
        }
    };

    (@mode Any)    => { crate::lowlevel::access::Mode::Single };
    (@mode Single) => { crate::lowlevel::access::Mode::Single };
    (@mode Burst)  => { crate::lowlevel::access::Mode::Burst };
}

/// Declare a category of command strobes (address-only triggers).
macro_rules! strobes {
    ($cat:ident, { $( $(#[$($rmeta:tt)*])* $NAME:ident @ $addr:literal ),* $(,)? }) => {
        pub mod $cat {
            $(
                $(#[$($rmeta)*])*
                #[allow(non_camel_case_types)]
                pub struct $NAME;
                impl crate::lowlevel::registers::Strobe for $NAME {
                    const ADDR: u8 = $addr;
                    const NO_EFFECT: bool = strobes!(@no_effect $NAME);
                }
            )*
        }
    };
    // SNOP is the only strobe with no effect, so its status byte is a valid read.
    (@no_effect SNOP) => { true };
    (@no_effect $other:ident) => { false };
}

/// Declare a category of multi-byte burst regions (FIFO, PATABLE).
macro_rules! burst {
    ($cat:ident, read = $rd:ident, write = $wr:ident, {
        $( $(#[$($rmeta:tt)*])* $NAME:ident @ $addr:literal ),* $(,)?
    }) => {
        pub mod $cat {
            $(
                $(#[$($rmeta)*])*
                #[allow(non_camel_case_types)]
                pub struct $NAME;
                burst!(@read $NAME, $addr, $rd);
                burst!(@write $NAME, $addr, $wr);
            )*
        }
    };
    (@read $NAME:ident, $addr:literal, Reject) => {};
    (@read $NAME:ident, $addr:literal, $mode:ident) => {
        impl crate::lowlevel::registers::BurstRead for $NAME { const ADDR: u8 = $addr; }
    };
    (@write $NAME:ident, $addr:literal, Reject) => {};
    (@write $NAME:ident, $addr:literal, $mode:ident) => {
        impl crate::lowlevel::registers::BurstWrite for $NAME { const ADDR: u8 = $addr; }
    };
}

// Register declarations live in per-category files. Each is a real module under a
// private `*_defs` name (so the macro's generated `pub mod <cat>` selector module
// doesn't nest as `config::config`), then glob-re-exported so the public paths stay
// flat: value structs at `registers::*`, selectors at
// `registers::{config,status,command,multi}::*`.
#[path = "registers/config.rs"]
mod config_defs;
pub use config_defs::*;

#[path = "registers/status.rs"]
mod status_defs;
pub use status_defs::*;

#[path = "registers/command.rs"]
mod command_defs;
pub use command_defs::*;

#[path = "registers/multi_byte.rs"]
mod multi_defs;
pub use multi_defs::*;
