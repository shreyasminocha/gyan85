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
    use crate::yan85::register::Register as Reg;

    #[test]
    fn test_assemble_imm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::IMM(Reg::C, 0x69)),
            [consts.opcode.IMM, consts.register.C, 0x69]
        )
    }

    #[test]
    fn test_assemble_add() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::ADD(Reg::B, Reg::S)),
            [consts.opcode.ADD, consts.register.B, consts.register.S]
        )
    }

    #[test]
    fn test_assemble_stk() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::STK(Reg::C, Reg::I)),
            [consts.opcode.STK, consts.register.C, consts.register.I]
        )
    }

    #[test]
    fn test_assemble_stm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::STM(Reg::C, Reg::D)),
            [consts.opcode.STM, consts.register.C, consts.register.D]
        )
    }

    #[test]
    fn test_assemble_ldm() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::LDM(Reg::B, Reg::C)),
            [consts.opcode.LDM, consts.register.B, consts.register.C]
        );
    }

    #[test]
    fn test_assemble_cmp() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(consts, &Instruction::CMP(Reg::C, Reg::D)),
            [consts.opcode.CMP, consts.register.C, consts.register.D]
        )
    }

    #[test]
    fn test_assemble_jmp() {
        let consts = Constants::default();
        assert_eq!(
            assemble_instruction(
                consts,
                &Instruction::JMP(consts.flag.L | consts.flag.G, Reg::D)
            ),
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
            assemble_instruction(consts, &Instruction::SYS(consts.syscall.WRITE, Reg::D)),
            [consts.opcode.SYS, consts.syscall.WRITE, consts.register.D]
        )
    }
}
