use anyhow::{bail, Result};

use crate::yan85::{
    constants::{Constants, Decodable},
    flags::Flags,
    instruction::Instruction,
    register::Register,
};

/// Attempts to convert the given bytes to Yan85 instructions.
pub fn disassemble(bytes: Vec<u8>, constants: Constants) -> Result<Vec<Instruction>> {
    bytes
        .chunks_exact(3)
        .map(|inst| {
            if let [a, b, op] = inst {
                disassemble_instruction([*a, *b, *op], constants)
            } else {
                unreachable!("Chunks should be of length 3")
            }
        })
        .collect()
}

/// Attempts to convert the given byte 3-tuple to a Yan85 instruction.
fn disassemble_instruction(bytes: [u8; 3], constants: Constants) -> Result<Instruction> {
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
        _ if op == o.STK => Ok(Instruction::STK(
            Option::<Register>::decode(a, constants)?,
            Option::<Register>::decode(b, constants)?,
        )),
        _ if op == o.STM => Ok(Instruction::STM(a_register?, b_register?)),
        _ if op == o.LDM => Ok(Instruction::LDM(a_register?, b_register?)),
        _ if op == o.CMP => Ok(Instruction::CMP(a_register?, b_register?)),
        _ if op == o.JMP => Ok(Instruction::JMP(Flags::decode(a, constants)?, b_register?)),
        _ if op == o.SYS => Ok(Instruction::SYS(
            a,
            Option::<Register>::decode(b, constants)?,
        )),
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
            disassemble_instruction([consts.opcode.IMM, consts.register.C, 0x69], consts).unwrap(),
            Instruction::IMM(Reg::C, 0x69)
        );
    }

    #[test]
    fn test_disassemble_add() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                [consts.opcode.ADD, consts.register.B, consts.register.S],
                consts
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
                [consts.opcode.STK, consts.register.C, consts.register.I],
                consts
            )
            .unwrap(),
            Instruction::STK(Some(Reg::C), Some(Reg::I))
        )
    }

    #[test]
    fn test_disassemble_stk_none_operand() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction([consts.opcode.STK, consts.register.C, 0], consts).unwrap(),
            Instruction::STK(Some(Reg::C), None)
        )
    }

    #[test]
    fn test_disassemble_stk_none_operands() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction([consts.opcode.STK, 0, 0], consts).unwrap(),
            Instruction::STK(None, None)
        )
    }

    #[test]
    fn test_disassemble_stm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                [consts.opcode.STM, consts.register.C, consts.register.D],
                consts
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
                [consts.opcode.LDM, consts.register.B, consts.register.B],
                consts
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
                [consts.opcode.CMP, consts.register.C, consts.register.D],
                consts
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
                [
                    consts.opcode.JMP,
                    consts.flag.L | consts.flag.G,
                    consts.register.D,
                ],
                consts
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
                [consts.opcode.SYS, consts.syscall.WRITE, consts.register.D],
                consts
            )
            .unwrap(),
            Instruction::SYS(consts.syscall.WRITE, Some(Reg::D)),
        );
    }

    #[test]
    fn test_disassemble_sys_none_operand() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction([consts.opcode.SYS, consts.syscall.EXIT, 0], consts).unwrap(),
            Instruction::SYS(consts.syscall.EXIT, None),
        );
    }
}
