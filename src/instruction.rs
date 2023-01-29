use std::fmt;

use crate::register::Register;

pub type SysCall = u8;
pub type Condition = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    IMM(Register, u8),
    ADD(Register, Register),
    STK(Register, Register),
    STM(Register, Register),
    LDM(Register, Register),
    CMP(Register, Register),
    JMP(Condition, Register),
    SYS(SysCall, Register),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::IMM(a, b) => write!(f, "IMM {a} = {b:#02x}"),
            Instruction::ADD(a, b) => write!(f, "ADD {a} {b}"),
            Instruction::STK(a, b) => write!(f, "STK {b} {a}"),
            Instruction::STM(a, b) => write!(f, "STM *{a} = {b}"),
            Instruction::LDM(a, b) => write!(f, "LDM {a} = *{b}"),
            Instruction::CMP(a, b) => write!(f, "CMP {b} {a}"),
            Instruction::JMP(a, b) => {
                let mut flags = "".to_string();

                if a & 1 != 0 {
                    flags.push('L');
                }
                if a & 8 != 0 {
                    flags.push('G');
                }
                if a & 4 != 0 {
                    flags.push('E');
                }
                if a & 0x10 != 0 {
                    flags.push('N');
                }
                if a & 2 != 0 {
                    flags.push('Z');
                }
                if *a == 0 {
                    flags.push('*');
                }

                write!(f, "JMP {flags} {b}")
            }
            Instruction::SYS(a, b) => write!(f, "SYS {a:#02x} {b}"),
        }
    }
}
