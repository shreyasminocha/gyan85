use crate::{
    constants::opcode::*,
    instruction::{Instruction, SysCall},
    register::Register,
};

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::IMM(register, value) => assemble_imm(*register, *value),
            Instruction::ADD(dest, operand) => assemble_add(*dest, *operand),
            Instruction::STK(push, pop) => assemble_stk(*push, *pop),
            Instruction::STM(dest, src) => assemble_stm(*dest, *src),
            Instruction::LDM(dest, src) => assemble_ldm(*dest, *src),
            Instruction::CMP(a, b) => assemble_cmp(*a, *b),
            Instruction::JMP(condition, register) => assemble_jmp(*condition, *register),
            Instruction::SYS(syscall, register) => assemble_sys(*syscall, *register),
        })
        .collect()
}

fn assemble_imm(register: Register, value: u8) -> [u8; 3] {
    [register as u8, value, IMM]
}

fn assemble_add(dest: Register, operand: Register) -> [u8; 3] {
    [dest as u8, operand as u8, ADD]
}

fn assemble_stk(push: Register, pop: Register) -> [u8; 3] {
    [pop as u8, push as u8, STK]
}

fn assemble_stm(dest: Register, src: Register) -> [u8; 3] {
    [dest as u8, src as u8, STM]
}

fn assemble_ldm(dest: Register, src: Register) -> [u8; 3] {
    [src as u8, dest as u8, LDM]
}

fn assemble_cmp(a: Register, b: Register) -> [u8; 3] {
    [b as u8, a as u8, CMP]
}

fn assemble_jmp(condition: u8, register: Register) -> [u8; 3] {
    [condition, register as u8, JMP]
}

fn assemble_sys(syscall: SysCall, register: Register) -> [u8; 3] {
    [syscall, register as u8, SYS]
}

#[cfg(test)]
mod tests {
    use crate::constants::{flag::*, register::*, syscall::*};

    use super::*;

    #[test]
    fn test_assemble_imm() {
        assert_eq!(assemble_imm(Register::C, 0x69), [C, 0x69, IMM])
    }

    #[test]
    fn test_assemble_add() {
        assert_eq!(assemble_add(Register::B, Register::S), [B, S, ADD])
    }

    #[test]
    fn test_assemble_stk() {
        assert_eq!(assemble_stk(Register::C, Register::I), [I, C, STK])
    }

    #[test]
    fn test_assemble_stm() {
        assert_eq!(assemble_stm(Register::C, Register::D), [C, D, STM])
    }

    #[test]
    fn test_assemble_ldm() {
        assert_eq!(assemble_ldm(Register::B, Register::B), [B, B, LDM]);
    }

    #[test]
    fn test_assemble_cmp() {
        assert_eq!(assemble_cmp(Register::C, Register::D), [D, C, CMP])
    }

    #[test]
    fn test_assemble_jmp() {
        assert_eq!(assemble_jmp(L | G, Register::D), [L | G, D, JMP]);
    }

    #[test]
    fn test_assemble_sys() {
        assert_eq!(assemble_sys(WRITE, Register::D), [WRITE, D, SYS])
    }
}
