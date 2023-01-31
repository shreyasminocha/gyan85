use crate::{constants::Constants, instruction::Instruction};

pub fn assemble(constants: Constants, instructions: Vec<Instruction>) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|i| assemble_instruction(constants, i))
        .collect()
}

fn assemble_instruction(c: Constants, instruction: &Instruction) -> [u8; 3] {
    let op = c.opcode;

    match instruction {
        Instruction::IMM(register, value) => [register.to_u8(c), *value, op.IMM],
        Instruction::ADD(dest, operand) => [dest.to_u8(c), operand.to_u8(c), op.ADD],
        Instruction::STK(push, pop) => [pop.to_u8(c), push.to_u8(c), op.STK],
        Instruction::STM(dest, src) => [dest.to_u8(c), src.to_u8(c), op.STM],
        Instruction::LDM(dest, src) => [src.to_u8(c), dest.to_u8(c), op.LDM],
        Instruction::CMP(a, b) => [b.to_u8(c), a.to_u8(c), op.CMP],
        Instruction::JMP(condition, register) => [*condition, register.to_u8(c), op.JMP],
        Instruction::SYS(syscall, register) => [*syscall, register.to_u8(c), op.SYS],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{Register, TEST_CONSTANTS as CONSTS, *},
        register::Register as Reg,
    };

    const R: Register = CONSTS.register;
    const O: Opcode = CONSTS.opcode;
    const F: Flag = CONSTS.flag;
    const S: Syscall = CONSTS.syscall;

    #[test]
    fn test_assemble_imm() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::IMM(Reg::C, 0x69)),
            [R.C, 0x69, O.IMM]
        )
    }

    #[test]
    fn test_assemble_add() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::ADD(Reg::B, Reg::S)),
            [R.B, R.S, O.ADD]
        )
    }

    #[test]
    fn test_assemble_stk() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::STK(Reg::C, Reg::I)),
            [R.I, R.C, O.STK]
        )
    }

    #[test]
    fn test_assemble_stm() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::STM(Reg::C, Reg::D)),
            [R.C, R.D, O.STM]
        )
    }

    #[test]
    fn test_assemble_ldm() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::LDM(Reg::B, Reg::B)),
            [R.B, R.B, O.LDM]
        );
    }

    #[test]
    fn test_assemble_cmp() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::CMP(Reg::C, Reg::D)),
            [R.D, R.C, O.CMP]
        )
    }

    #[test]
    fn test_assemble_jmp() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::JMP(F.L | F.G, Reg::D)),
            [F.L | F.G, R.D, O.JMP]
        );
    }

    #[test]
    fn test_assemble_sys() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::SYS(S.WRITE, Reg::D)),
            [S.WRITE, R.D, O.SYS]
        )
    }
}
