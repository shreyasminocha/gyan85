use crate::yan85::{
    constants::{Constants, Encodable},
    instruction::Instruction,
};

/// Assembles the given instructions, converting them into bytes.
pub fn assemble(instructions: &[Instruction], constants: Constants) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|i| assemble_instruction(i, constants))
        .collect()
}

/// Assembles the given instruction, converting it into its three-byte data representation.
fn assemble_instruction(instruction: &Instruction, c: Constants) -> [u8; 3] {
    let o = c.opcode;
    let bo = c.byte_order;

    let [op, a, b] = match instruction {
        Instruction::IMM(register, value) => [o.IMM, register.encode(c), *value],
        Instruction::ADD(dest, operand) => [o.ADD, dest.encode(c), operand.encode(c)],
        Instruction::STK(pop, push) => [o.STK, pop.encode(c), push.encode(c)],
        Instruction::STM(dest, src) => [o.STM, dest.encode(c), src.encode(c)],
        Instruction::LDM(dest, src) => [o.LDM, dest.encode(c), src.encode(c)],
        Instruction::CMP(a, b) => [o.CMP, a.encode(c), b.encode(c)],
        Instruction::JMP(condition, register) => [o.JMP, condition.encode(c), register.encode(c)],
        Instruction::SYS(syscall, register) => [o.SYS, *syscall, register.encode(c)],
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
    use crate::yan85::register::Register as Reg;

    #[test]
    fn test_assemble_imm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::IMM(Reg::C, 0x69), consts),
            [consts.opcode.IMM, consts.register.C, 0x69]
        )
    }

    #[test]
    fn test_assemble_add() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::ADD(Reg::B, Reg::S), consts),
            [consts.opcode.ADD, consts.register.B, consts.register.S]
        )
    }

    #[test]
    fn test_assemble_stk() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::STK(Some(Reg::C), Some(Reg::I)), consts),
            [consts.opcode.STK, consts.register.C, consts.register.I]
        )
    }

    #[test]
    fn test_assemble_stk_none_operand() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::STK(Some(Reg::C), None), consts),
            [consts.opcode.STK, consts.register.C, 0]
        )
    }

    #[test]
    fn test_assemble_stk_none_operands() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::STK(None, None), consts),
            [consts.opcode.STK, 0, 0]
        )
    }

    #[test]
    fn test_assemble_stm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::STM(Reg::C, Reg::D), consts),
            [consts.opcode.STM, consts.register.C, consts.register.D]
        )
    }

    #[test]
    fn test_assemble_ldm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::LDM(Reg::B, Reg::C), consts),
            [consts.opcode.LDM, consts.register.B, consts.register.C]
        );
    }

    #[test]
    fn test_assemble_cmp() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::CMP(Reg::C, Reg::D), consts),
            [consts.opcode.CMP, consts.register.C, consts.register.D]
        )
    }

    #[test]
    fn test_assemble_jmp() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::JMP("LG".try_into().unwrap(), Reg::D), consts),
            [
                consts.opcode.JMP,
                consts.flag.L | consts.flag.G,
                consts.register.D,
            ]
        );
    }

    #[test]
    fn test_assemble_sys() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(&Instruction::SYS(consts.syscall.WRITE, Reg::D), consts),
            [consts.opcode.SYS, consts.syscall.WRITE, consts.register.D]
        )
    }
}
