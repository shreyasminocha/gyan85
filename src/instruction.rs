use colored::Colorize;
use std::fmt;

use crate::{constants::flag::*, register::Register};

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
            Instruction::IMM(a, b) => {
                write!(f, "{} {a} = {}", "IMM".green(), format!("{b:#02x}").blue())
            }
            Instruction::ADD(a, b) => write!(f, "{} {a} {b}", "ADD".green()),
            Instruction::STK(a, b) => write!(f, "{} {b} {a}", "STK".green()),
            Instruction::STM(a, b) => write!(f, "{} *{a} = {b}", "STM".green()),
            Instruction::LDM(a, b) => write!(f, "{} {a} = *{b}", "LDM".green()),
            Instruction::CMP(a, b) => write!(f, "{} {b} {a}", "CMP".green()),
            Instruction::JMP(a, b) => {
                let mut flags = "".to_string();

                if a & L != 0 {
                    flags.push('L');
                }
                if a & G != 0 {
                    flags.push('G');
                }
                if a & E != 0 {
                    flags.push('E');
                }
                if a & N != 0 {
                    flags.push('N');
                }
                if a & Z != 0 {
                    flags.push('Z');
                }
                if *a == 0 {
                    flags.push('*');
                }

                write!(f, "{} {} {b}", "JMP".green(), flags.blue())
            }
            Instruction::SYS(a, b) => {
                write!(f, "{} {} {b}", "SYS".green(), format!("{a:#02x}").blue())
            }
        }
    }
}
