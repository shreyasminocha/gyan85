use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register {
    A = 0x20,
    B = 0x40,
    C = 0x8,
    D = 0x2,
    S = 0x10,
    I = 0x4,
    F = 0x1,
    None = 0x0,
}

impl Register {
    pub fn try_from(register: u8) -> Option<Register> {
        match register {
            0x20 => Some(Register::A),
            0x40 => Some(Register::B),
            0x8 => Some(Register::C),
            0x2 => Some(Register::D),
            0x10 => Some(Register::S),
            0x4 => Some(Register::I),
            0x0 => Some(Register::None),
            _ => None,
        }
    }

    pub fn to_index(self) -> usize {
        (self as u8).trailing_zeros() as usize
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::A => write!(f, "a"),
            Register::B => write!(f, "b"),
            Register::C => write!(f, "c"),
            Register::D => write!(f, "d"),
            Register::S => write!(f, "s"),
            Register::I => write!(f, "i"),
            Register::F => write!(f, "f"),
            Register::None => write!(f, "NONE"),
        }
    }
}
