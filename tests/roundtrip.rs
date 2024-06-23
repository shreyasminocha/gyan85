use gyan85::{
    asm::assemble,
    disasm::disassemble,
    yan85::{constants::TEST_CONSTANTS, instruction::Instruction, register::Register},
};

#[test]
fn test_imm() {
    let instructions = vec![Instruction::IMM(Register::A, 5)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_add() {
    let instructions = vec![Instruction::ADD(Register::A, Register::B)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_push() {
    let instructions = vec![Instruction::STK(Register::None, Register::A)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_pop() {
    let instructions = vec![Instruction::STK(Register::A, Register::None)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_push_pop() {
    let instructions = vec![Instruction::STK(Register::A, Register::B)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_ldm() {
    let instructions = vec![Instruction::LDM(Register::A, Register::B)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_cmp() {
    let instructions = vec![Instruction::CMP(Register::A, Register::B)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_jmp() {
    let instructions = vec![Instruction::JMP(TEST_CONSTANTS.flag.L, Register::A)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}

#[test]
fn test_sys() {
    let instructions = vec![Instruction::SYS(0x1, Register::D)];
    let bytes = assemble(TEST_CONSTANTS, &instructions);

    assert_eq!(disassemble(TEST_CONSTANTS, bytes).unwrap(), instructions);
}
