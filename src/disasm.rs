use crate::{instruction::Instruction, register::Register};

#[derive(Debug, PartialEq, Eq)]
pub struct DisassembleError;

pub fn disassemble(bytes: Vec<u8>) -> Result<Vec<Instruction>, DisassembleError> {
    bytes
        .chunks_exact(3)
        .map(|inst| {
            if let [a, b, op] = inst {
                disassemble_instruction([*a, *b, *op])
            } else {
                Err(DisassembleError)
            }
        })
        .collect()
}

fn disassemble_instruction(bytes: [u8; 3]) -> Result<Instruction, DisassembleError> {
    let [a, b, op] = bytes;
    let a_register = Register::try_from(a);
    let b_register = Register::try_from(b);

    match op {
        0x1 => Ok(Instruction::IMM(a_register.expect("Invalid register"), b)),
        0x2 => Ok(Instruction::ADD(
            a_register.expect("Invalid register"),
            b_register.expect("Invalid register"),
        )),
        0x4 => Ok(Instruction::SYS(a, b_register.expect("Invalid register"))),
        0x8 => Ok(Instruction::CMP(
            b_register.expect("Invalid register"),
            a_register.expect("Invalid register"),
        )),
        0x10 => Ok(Instruction::STM(
            a_register.expect("Invalid register"),
            b_register.expect("Invalid register"),
        )),
        0x20 => Ok(Instruction::LDM(
            a_register.expect("Invalid register"),
            b_register.expect("Invalid register"),
        )),
        0x40 => Ok(Instruction::JMP(a, b_register.expect("Invalid register"))),
        0x80 => Ok(Instruction::STK(
            b_register.expect("Invalid register"),
            a_register.expect("Invalid register"),
        )),
        _ => Err(DisassembleError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_imm() {
        assert_eq!(
            disassemble_instruction([0x08, 0x69, 0x01]),
            Ok(Instruction::IMM(Register::C, 0x69))
        );
    }

    #[test]
    fn test_disassemble_stk() {
        assert_eq!(
            disassemble_instruction([0x04, 0x08, 0x80]),
            Ok(Instruction::STK(Register::C, Register::I))
        )
    }

    #[test]
    fn test_disassemble_stm() {
        assert_eq!(
            disassemble_instruction([0x08, 0x02, 0x10]),
            Ok(Instruction::STM(Register::C, Register::D)),
        );
    }

    #[test]
    fn test_disassemble_sys() {
        assert_eq!(
            disassemble_instruction([0x01, 0x02, 0x04]),
            Ok(Instruction::SYS(0x1, Register::D)),
        );
    }

    #[test]
    fn test_disassemble_add() {
        assert_eq!(
            disassemble_instruction([0x40, 0x10, 0x02]),
            Ok(Instruction::ADD(Register::B, Register::S)),
        );
    }

    #[test]
    fn test_disassemble_cmp() {
        assert_eq!(
            disassemble_instruction([0x08, 0x02, 0x08]),
            Ok(Instruction::CMP(Register::D, Register::C)),
        );
    }

    #[test]
    fn test_disassemble_ldm() {
        assert_eq!(
            disassemble_instruction([0x40, 0x40, 0x20]),
            Ok(Instruction::LDM(Register::B, Register::B)),
        );
    }

    #[test]
    fn test_disassemble_jmp() {
        assert_eq!(
            disassemble_instruction([0x09, 0x02, 0x40]),
            Ok(Instruction::JMP(0x9, Register::D)),
        );
    }
}
