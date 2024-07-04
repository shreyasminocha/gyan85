use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fmt;

use super::constants::{Constants, Decodable, Encodable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
/// Yan85 registers.
pub enum Register {
    /// General-purpose register "a".
    A,
    /// General-purpose register "b".
    B,
    /// General-purpose register "c".
    C,
    /// General-purpose register "d".
    D,
    /// Stack pointer.
    S,
    /// Instruction pointer.
    I,
    /// Flag register. Typically modified via CMP instructions and implicitly used in JMP
    /// instructions.
    F,
}

impl Encodable for Register {
    fn encode(&self, c: Constants) -> u8 {
        match self {
            Register::A => c.register.A,
            Register::B => c.register.B,
            Register::C => c.register.C,
            Register::D => c.register.D,
            Register::S => c.register.S,
            Register::I => c.register.I,
            Register::F => c.register.F,
        }
    }
}

impl Decodable for Register {
    fn decode(value: u8, c: Constants) -> Result<Self> {
        match value {
            _ if value == c.register.A => Ok(Register::A),
            _ if value == c.register.B => Ok(Register::B),
            _ if value == c.register.C => Ok(Register::C),
            _ if value == c.register.D => Ok(Register::D),
            _ if value == c.register.S => Ok(Register::S),
            _ if value == c.register.I => Ok(Register::I),
            _ if value == c.register.F => Ok(Register::F),
            _ => Err(anyhow!("Invalid register: {value:#02x}")),
        }
    }
}

impl Encodable for Option<Register> {
    fn encode(&self, c: Constants) -> u8 {
        match self {
            Some(Register::A) => c.register.A,
            Some(Register::B) => c.register.B,
            Some(Register::C) => c.register.C,
            Some(Register::D) => c.register.D,
            Some(Register::S) => c.register.S,
            Some(Register::I) => c.register.I,
            Some(Register::F) => c.register.F,
            None => 0x0,
        }
    }
}

impl Decodable for Option<Register> {
    fn decode(value: u8, c: Constants) -> Result<Self> {
        match value {
            _ if value == c.register.A => Ok(Some(Register::A)),
            _ if value == c.register.B => Ok(Some(Register::B)),
            _ if value == c.register.C => Ok(Some(Register::C)),
            _ if value == c.register.D => Ok(Some(Register::D)),
            _ if value == c.register.S => Ok(Some(Register::S)),
            _ if value == c.register.I => Ok(Some(Register::I)),
            _ if value == c.register.F => Ok(Some(Register::F)),
            0x0 => Ok(None),
            _ => Err(anyhow!("Invalid register: {value:#02x}")),
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
        }
    }
}
