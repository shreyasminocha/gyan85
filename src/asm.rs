use crate::{constants::opcode::*, instruction::Instruction};

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u8> {
    instructions.iter().flat_map(assemble_instruction).collect()
}

fn assemble_instruction(instruction: &Instruction) -> [u8; 3] {
    match instruction {
        Instruction::IMM(register, value) => [*register as u8, *value, IMM],
        Instruction::ADD(dest, operand) => [*dest as u8, *operand as u8, ADD],
        Instruction::STK(push, pop) => [*pop as u8, *push as u8, STK],
        Instruction::STM(dest, src) => [*dest as u8, *src as u8, STM],
        Instruction::LDM(dest, src) => [*src as u8, *dest as u8, LDM],
        Instruction::CMP(a, b) => [*b as u8, *a as u8, CMP],
        Instruction::JMP(condition, register) => [*condition, *register as u8, JMP],
        Instruction::SYS(syscall, register) => [*syscall, *register as u8, SYS],
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{flag::*, register::*, syscall::*},
        register::Register,
    };

    use super::*;

    #[test]
    fn test_assemble_imm() {
        assert_eq!(
            assemble_instruction(&Instruction::IMM(Register::C, 0x69)),
            [C, 0x69, IMM]
        )
    }

    #[test]
    fn test_assemble_add() {
        assert_eq!(
            assemble_instruction(&Instruction::ADD(Register::B, Register::S)),
            [B, S, ADD]
        )
    }

    #[test]
    fn test_assemble_stk() {
        assert_eq!(
            assemble_instruction(&Instruction::STK(Register::C, Register::I)),
            [I, C, STK]
        )
    }

    #[test]
    fn test_assemble_stm() {
        assert_eq!(
            assemble_instruction(&Instruction::STM(Register::C, Register::D)),
            [C, D, STM]
        )
    }

    #[test]
    fn test_assemble_ldm() {
        assert_eq!(
            assemble_instruction(&Instruction::LDM(Register::B, Register::B)),
            [B, B, LDM]
        );
    }

    #[test]
    fn test_assemble_cmp() {
        assert_eq!(
            assemble_instruction(&Instruction::CMP(Register::C, Register::D)),
            [D, C, CMP]
        )
    }

    #[test]
    fn test_assemble_jmp() {
        assert_eq!(
            assemble_instruction(&Instruction::JMP(L | G, Register::D)),
            [L | G, D, JMP]
        );
    }

    #[test]
    fn test_assemble_sys() {
        assert_eq!(
            assemble_instruction(&Instruction::SYS(WRITE, Register::D)),
            [WRITE, D, SYS]
        )
    }
}
