use colored::Colorize;
use std::fmt;

use crate::yan85::register::Register;

/// Yan85 syscall number.
pub type SysCall = u8;

/// Jump condition specifier.
pub type Condition = u8;

/// Mutable register.
pub type MutRegister = Register;

/// A register that holds a memory pointer.
pub type PointerRegister = Register;

/// Yan85 instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// Immediate instruction that assigns an integer to a register.
    IMM(Register, u8),
    /// Adds two registers.
    ///
    /// `ADD a b` increments the value in register `a` by the value in register `b`.
    ADD(MutRegister, Register),
    /// Stack instruction that performs stack operations.
    ///
    /// Pops values from the stack into a register and pushes values from registers into the stack.
    STK(Register, Register),
    /// Assigns a value to a memory location.
    ///
    /// `STM *a = b` assigns the value in `b` to the memory location referenced by `a`.
    STM(PointerRegister, Register),
    /// "Load from memory" instruction.
    ///
    /// `LDM a = *b` assigns the value in the memory location referenced by `b` to register `a`.
    LDM(MutRegister, PointerRegister),
    /// Comparison instruction.
    CMP(Register, Register),
    /// Jumps to the instruction referenced by a register if the specified condition is met by the
    /// value in the "flag" register.
    JMP(Condition, Register),
    /// Syscall instruction.
    SYS(SysCall, Register),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::IMM(a, b) => {
                write!(f, "{} {a} = {}", "IMM".green(), format!("{b:#02x}").blue())
            }
            Instruction::ADD(a, b) => write!(f, "{} {a} {b}", "ADD".green()),
            Instruction::STK(a, b) => write!(f, "{} {a} {b}", "STK".green()),
            Instruction::STM(a, b) => write!(f, "{} *{a} = {b}", "STM".green()),
            Instruction::LDM(a, b) => write!(f, "{} {a} = *{b}", "LDM".green()),
            Instruction::CMP(a, b) => write!(f, "{} {a} {b}", "CMP".green()),
            Instruction::JMP(a, b) => {
                // TODO: access the flag constants here and re-add the flag description
                write!(f, "{} {} {b}", "JMP".green(), a.to_string().blue())
            }
            Instruction::SYS(a, b) => {
                write!(f, "{} {} {b}", "SYS".green(), format!("{a:#02x}").blue())
            }
        }
    }
}
