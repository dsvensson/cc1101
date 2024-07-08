mod command;
mod config;
mod multi_byte;
mod status;
mod status_byte;

pub use self::command::*;
pub use self::config::*;
pub use self::multi_byte::*;
pub use self::status::*;
pub use self::status_byte::*;

use crate::lowlevel::access;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Register {
    Command(command::Command),
    Config(config::Config),
    MultiByte(multi_byte::MultiByte),
    Status(status::Status),
}

impl Register {
    pub fn raddr(self, mode: access::Mode) -> u8 {
        match self {
            Register::Command(r) => r.addr(access::Access::Read, access::Mode::Single),
            Register::Config(r) => r.addr(access::Access::Read, mode),
            Register::MultiByte(r) => r.addr(access::Access::Read, mode),
            Register::Status(r) => r.addr(access::Access::Read, access::Mode::Burst),
        }
    }

    pub fn waddr(self, mode: access::Mode) -> u8 {
        match self {
            Register::Command(r) => r.addr(access::Access::Write, access::Mode::Single),
            Register::Config(r) => r.addr(access::Access::Write, mode),
            Register::MultiByte(r) => r.addr(access::Access::Write, mode),
            Register::Status(_r) => panic!("Status cannot be written!"),
        }
    }
}
