use crate::{
    instruction::{Instruction, SysCall},
    register::Register,
};

const IMM: u8 = 0x1;
const ADD: u8 = 0x2;
const SYS: u8 = 0x4;
const CMP: u8 = 0x8;
const STM: u8 = 0x10;
const LDM: u8 = 0x20;
const JMP: u8 = 0x40;
const STK: u8 = 0x80;

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::IMM(register, value) => assemble_imm(*register, *value),
            Instruction::ADD(dest, operand) => assemble_add(*dest, *operand),
            Instruction::SYS(syscall, register) => assemble_sys(*syscall, *register),
            Instruction::CMP(a, b) => assemble_cmp(*a, *b),
            Instruction::STM(dest, src) => assemble_stm(*dest, *src),
            Instruction::LDM(dest, src) => assemble_ldm(*dest, *src),
            Instruction::JMP(condition, register) => assemble_jmp(*condition, *register),
            Instruction::STK(push, pop) => assemble_stk(*push, *pop),
        })
        .collect()
}

fn assemble_imm(register: Register, value: u8) -> [u8; 3] {
    [register as u8, value, IMM]
}

fn assemble_stk(push: Register, pop: Register) -> [u8; 3] {
    [pop as u8, push as u8, STK]
}

fn assemble_stm(dest: Register, src: Register) -> [u8; 3] {
    [dest as u8, src as u8, STM]
}

fn assemble_sys(syscall: SysCall, register: Register) -> [u8; 3] {
    [syscall, register as u8, SYS]
}

fn assemble_add(dest: Register, operand: Register) -> [u8; 3] {
    [dest as u8, operand as u8, ADD]
}

fn assemble_cmp(a: Register, b: Register) -> [u8; 3] {
    [b as u8, a as u8, CMP]
}

fn assemble_jmp(condition: u8, register: Register) -> [u8; 3] {
    [condition, register as u8, JMP]
}

// TODO: confirm argument order
fn assemble_ldm(dest: Register, src: Register) -> [u8; 3] {
    [src as u8, dest as u8, LDM]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_imm() {
        assert_eq!(assemble_imm(Register::C, 0x69), [0x08, 0x69, 0x01])
    }

    #[test]
    fn test_assemble_stk() {
        assert_eq!(assemble_stk(Register::C, Register::I), [0x04, 0x08, 0x80])
    }

    #[test]
    fn test_assemble_stm() {
        assert_eq!(assemble_stm(Register::C, Register::D), [0x08, 0x02, 0x10])
    }

    #[test]
    fn test_assemble_sys() {
        assert_eq!(assemble_sys(0x1, Register::D), [0x01, 0x02, 0x04])
    }

    #[test]
    fn test_assemble_add() {
        assert_eq!(assemble_add(Register::B, Register::S), [0x40, 0x10, 0x02])
    }

    #[test]
    fn test_assemble_cmp() {
        assert_eq!(assemble_cmp(Register::D, Register::C), [0x08, 0x02, 0x08])
    }

    #[test]
    fn test_assemble_ldm() {
        assert_eq!(assemble_ldm(Register::B, Register::B), [0x40, 0x40, 0x20]);
    }

    #[test]
    fn test_assemble_jmp() {
        assert_eq!(assemble_jmp(0x9, Register::D), [0x09, 0x02, 0x40]);
    }
}
