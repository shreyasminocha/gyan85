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
