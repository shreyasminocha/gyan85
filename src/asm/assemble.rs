use crate::yan85::{constants::Constants, instruction::Instruction};

/// Assembles the given instructions, converting them into bytes.
pub fn assemble(constants: Constants, instructions: &[Instruction]) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|i| assemble_instruction(constants, i))
        .collect()
}

/// Assembles the given instruction, converting it into its three-byte data representation.
fn assemble_instruction(c: Constants, instruction: &Instruction) -> [u8; 3] {
    let o = c.opcode;
    let bo = c.byte_order;

    let [op, a, b] = match instruction {
        Instruction::IMM(register, value) => [o.IMM, register.to_u8(c), *value],
        Instruction::ADD(dest, operand) => [o.ADD, dest.to_u8(c), operand.to_u8(c)],
        Instruction::STK(pop, push) => [o.STK, pop.to_u8(c), push.to_u8(c)],
        Instruction::STM(dest, src) => [o.STM, dest.to_u8(c), src.to_u8(c)],
        Instruction::LDM(dest, src) => [o.LDM, dest.to_u8(c), src.to_u8(c)],
        Instruction::CMP(a, b) => [o.CMP, a.to_u8(c), b.to_u8(c)],
        Instruction::JMP(condition, register) => [o.JMP, *condition, register.to_u8(c)],
        Instruction::SYS(syscall, register) => [o.SYS, *syscall, register.to_u8(c)],
    };

    let mut data = [0; 3];
    data[bo.op as usize] = op;
    data[bo.a as usize] = a;
    data[bo.b as usize] = b;

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yan85::{
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
            [R.C, R.I, O.STK]
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
            assemble_instruction(CONSTS, &Instruction::LDM(Reg::B, Reg::C)),
            [R.B, R.C, O.LDM]
        );
    }

    #[test]
    fn test_assemble_cmp() {
        assert_eq!(
            assemble_instruction(CONSTS, &Instruction::CMP(Reg::C, Reg::D)),
            [R.C, R.D, O.CMP]
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
