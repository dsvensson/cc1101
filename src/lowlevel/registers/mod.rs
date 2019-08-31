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

pub trait RegisterClass {
    const REGISTER_CLASS: Register;

    fn bits(self) -> u8;
}

pub trait ReadableRegisterClass: From<u8> + Into<u8> + RegisterClass {
    type Writable: RegisterClass;
}

pub trait WritableRegisterClass: RegisterClass {}
