mod command;
mod config;
mod status;

pub use self::command::*;
pub use self::config::*;
pub use self::status::*;

use lowlevel::access;

#[derive(Clone, Copy)]
pub enum Register {
    Command(command::Command),
    Config(config::Config),
    Status(status::Status),
}

impl Register {
    pub fn raddr(self) -> u8 {
        0x80 | match self {
            Register::Command(r) => access::Mode::Single.offset(r.addr()),
            Register::Config(r) => access::Mode::Single.offset(r.addr()),
            Register::Status(r) => access::Mode::Burst.offset(r.addr()),
        }
    }

    pub fn waddr(self) -> u8 {
        match self {
            Register::Command(r) => access::Mode::Single.offset(r.addr()),
            Register::Config(r) => access::Mode::Single.offset(r.addr()),
            Register::Status(r) => access::Mode::Burst.offset(r.addr()),
        }
    }
}
