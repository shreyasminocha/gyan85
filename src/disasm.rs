use std::{error::Error, fmt::Display};

use crate::yan85::{constants::Constants, instruction::Instruction, register::Register};

#[derive(Debug, PartialEq, Eq)]
pub struct DisassembleError(pub String);

impl Display for DisassembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DisassembleError {}

pub fn disassemble(
    constants: Constants,
    bytes: Vec<u8>,
) -> Result<Vec<Instruction>, DisassembleError> {
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

fn disassemble_instruction(
    constants: Constants,
    bytes: [u8; 3],
) -> Result<Instruction, DisassembleError> {
    let bo = constants.byte_order;
    let o = constants.opcode;

    let op = bytes[bo.op as usize];
    let a = bytes[bo.a as usize];
    let b = bytes[bo.b as usize];

    let a_register = Register::try_from(a, constants);
    let b_register = Register::try_from(b, constants);

    match op {
        _ if op == o.IMM => Ok(Instruction::IMM(a_register?, b)),
        _ if op == o.ADD => Ok(Instruction::ADD(a_register?, b_register?)),
        _ if op == o.STK => Ok(Instruction::STK(a_register?, b_register?)),
        _ if op == o.STM => Ok(Instruction::STM(a_register?, b_register?)),
        _ if op == o.LDM => Ok(Instruction::LDM(a_register?, b_register?)),
        _ if op == o.CMP => Ok(Instruction::CMP(a_register?, b_register?)),
        _ if op == o.JMP => Ok(Instruction::JMP(a, b_register?)),
        _ if op == o.SYS => Ok(Instruction::SYS(a, b_register?)),
        _ => Err(DisassembleError(format!("Invalid opcode: {op:#02x}"))),
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
            disassemble_instruction(consts, [consts.opcode.IMM, consts.register.C, 0x69]),
            Ok(Instruction::IMM(Reg::C, 0x69))
        );
    }

    #[test]
    fn test_disassemble_add() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.ADD, consts.register.B, consts.register.S,]
            ),
            Ok(Instruction::ADD(Reg::B, Reg::S)),
        );
    }

    #[test]
    fn test_disassemble_stk() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.STK, consts.register.C, consts.register.I,]
            ),
            Ok(Instruction::STK(Reg::C, Reg::I))
        )
    }

    #[test]
    fn test_disassemble_stm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.STM, consts.register.C, consts.register.D,]
            ),
            Ok(Instruction::STM(Reg::C, Reg::D)),
        );
    }

    #[test]
    fn test_disassemble_ldm() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.LDM, consts.register.B, consts.register.B,]
            ),
            Ok(Instruction::LDM(Reg::B, Reg::B)),
        );
    }

    #[test]
    fn test_disassemble_cmp() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.CMP, consts.register.C, consts.register.D,]
            ),
            Ok(Instruction::CMP(Reg::C, Reg::D)),
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
            ),
            Ok(Instruction::JMP(consts.flag.L | consts.flag.G, Reg::D)),
        );
    }

    #[test]
    fn test_disassemble_sys() {
        let consts = Constants::default();
        assert_eq!(
            disassemble_instruction(
                consts,
                [consts.opcode.SYS, consts.syscall.WRITE, consts.register.D,]
            ),
            Ok(Instruction::SYS(consts.syscall.WRITE, Reg::D)),
        );
    }
}
