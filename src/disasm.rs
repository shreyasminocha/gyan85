use anyhow::{bail, Result};

use crate::yan85::{
    constants::{Constants, Decodable},
    flags::Flags,
    instruction::Instruction,
    register::Register,
};

/// Attempts to convert the given bytes to Yan85 instructions.
pub fn disassemble(constants: Constants, bytes: Vec<u8>) -> Result<Vec<Instruction>> {
    bytes
        .chunks_exact(3)
        .map(|inst| {
            if let [a, b, op] = inst {
                disassemble_instruction(constants, [*a, *b, *op])
            } else {
                unreachable!("Chunks should be of length 3")
            }
        })
        .collect()
}

/// Attempts to convert the given byte 3-tuple to a Yan85 instruction.
fn disassemble_instruction(constants: Constants, bytes: [u8; 3]) -> Result<Instruction> {
    let bo = constants.byte_order;
    let o = constants.opcode;

    let op = bytes[bo.op as usize];
    let a = bytes[bo.a as usize];
    let b = bytes[bo.b as usize];

    let a_register = Register::decode(a, constants);
    let b_register = Register::decode(b, constants);

    match op {
        _ if op == o.IMM => Ok(Instruction::IMM(a_register?, b)),
        _ if op == o.ADD => Ok(Instruction::ADD(a_register?, b_register?)),
        _ if op == o.STK => Ok(Instruction::STK(a_register?, b_register?)),
        _ if op == o.STM => Ok(Instruction::STM(a_register?, b_register?)),
        _ if op == o.LDM => Ok(Instruction::LDM(a_register?, b_register?)),
        _ if op == o.CMP => Ok(Instruction::CMP(a_register?, b_register?)),
        _ if op == o.JMP => Ok(Instruction::JMP(Flags::decode(a, constants)?, b_register?)),
        _ if op == o.SYS => Ok(Instruction::SYS(a, b_register?)),
        _ => bail!("Invalid opcode: {op:#02x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yan85::register::Register as Reg;

    #[test]
    fn test_disassemble_imm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(consts, [consts.opcode.IMM, consts.register.C, 0x69]).unwrap(),
            Instruction::IMM(Reg::C, 0x69)
        );
    }

    #[test]
    fn test_disassemble_add() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.ADD, consts.register.B, consts.register.S,]
            )
            .unwrap(),
            Instruction::ADD(Reg::B, Reg::S),
        );
    }

    #[test]
    fn test_disassemble_stk() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.STK, consts.register.C, consts.register.I,]
            )
            .unwrap(),
            Instruction::STK(Reg::C, Reg::I)
        )
    }

    #[test]
    fn test_disassemble_stm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.STM, consts.register.C, consts.register.D,]
            )
            .unwrap(),
            Instruction::STM(Reg::C, Reg::D),
        );
    }

    #[test]
    fn test_disassemble_ldm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.LDM, consts.register.B, consts.register.B,]
            )
            .unwrap(),
            Instruction::LDM(Reg::B, Reg::B),
        );
    }

    #[test]
    fn test_disassemble_cmp() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.CMP, consts.register.C, consts.register.D,]
            )
            .unwrap(),
            Instruction::CMP(Reg::C, Reg::D),
        );
    }

    #[test]
    fn test_disassemble_jmp() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [
                    consts.opcode.JMP,
                    consts.flag.L | consts.flag.G,
                    consts.register.D,
                ]
            )
            .unwrap(),
            Instruction::JMP("LG".try_into().unwrap(), Reg::D),
        );
    }

    #[test]
    fn test_disassemble_sys() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.SYS, consts.syscall.WRITE, consts.register.D,]
            )
            .unwrap(),
            Instruction::SYS(consts.syscall.WRITE, Reg::D),
        );
    }
}
