use colored::Colorize;
use std::fmt;

use crate::{constants::Constants, disasm::DisassembleError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    D,
    S,
    I,
    F,
    None,
}

impl Register {
    pub fn try_from(register: u8, constants: Constants) -> Result<Register, DisassembleError> {
        let r = constants.register;

        match register {
            _ if register == r.A => Ok(Register::A),
            _ if register == r.B => Ok(Register::B),
            _ if register == r.C => Ok(Register::C),
            _ if register == r.D => Ok(Register::D),
            _ if register == r.S => Ok(Register::S),
            _ if register == r.I => Ok(Register::I),
            _ if register == r.F => Ok(Register::F),
            0x0 => Ok(Register::None),
            _ => Err(DisassembleError("Invalid register".to_string())),
        }
    }

    pub fn to_u8(self, constants: Constants) -> u8 {
        let r = constants.register;

        match self {
            Register::A => r.A,
            Register::B => r.B,
            Register::C => r.C,
            Register::D => r.D,
            Register::S => r.S,
            Register::I => r.I,
            Register::F => r.F,
            Register::None => 0x0,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::A => write!(f, "{}", "a".red()),
            Register::B => write!(f, "{}", "b".red()),
            Register::C => write!(f, "{}", "c".red()),
            Register::D => write!(f, "{}", "d".red()),
            Register::S => write!(f, "{}", "s".red()),
            Register::I => write!(f, "{}", "i".red()),
            Register::F => write!(f, "{}", "f".red()),
            Register::None => write!(f, "{}", "NONE".black()),
        }
    }
}
