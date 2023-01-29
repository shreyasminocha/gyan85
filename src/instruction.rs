use std::fmt;

use crate::register::Register;

pub type SysCall = u8;
pub type Condition = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    IMM(Register, u8),
    ADD(Register, Register),
    SYS(SysCall, Register),
    CMP(Register, Register),
    STM(Register, Register),
    LDM(Register, Register),
    JMP(Condition, Register),
    STK(Register, Register),
}

const L: u8 = 0x11;
const E: u8 = 0x4;
const G: u8 = 0x18;
const N: u8 = L | G;
const LE: u8 = L | E;
const GE: u8 = G | E;
const LEG: u8 = L | E | G;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::IMM(a, b) => write!(f, "IMM {a} = {b:#02x}"),
            Instruction::ADD(a, b) => write!(f, "ADD {a} {b}"),
            Instruction::SYS(a, b) => write!(f, "SYS {a:#02x} {b}"),
            Instruction::CMP(a, b) => write!(f, "CMP {b} {a}"),
            Instruction::STM(a, b) => write!(f, "STM *{a} = {b}"),
            Instruction::LDM(a, b) => write!(f, "LDM {a} = *{b}"),
            Instruction::JMP(a, b) => {
                let val = format!("{:#02x}", *a);
                // TODO: fix the 'u8 to readable string' conversion
                let cond = match *a {
                    E => "E",
                    L => "L",
                    G => "G",
                    N => "N",
                    LE => "LE",
                    GE => "GE",
                    LEG => "LEG",
                    _ => &val,
                };
                write!(f, "JMP {cond} {b}")
            }
            Instruction::STK(a, b) => write!(f, "STK {b} {a}"),
        }
    }
}
