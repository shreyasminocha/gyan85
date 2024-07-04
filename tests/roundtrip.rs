use gyan85::{
    asm::assemble,
    disasm::disassemble,
    yan85::{constants::Constants, instruction::Instruction, register::Register},
};

#[test]
fn test_imm() {
    let consts = Constants::default();
    let instructions = vec![Instruction::IMM(Register::A, 5)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_add() {
    let consts = Constants::default();
    let instructions = vec![Instruction::ADD(Register::A, Register::B)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_push() {
    let consts = Constants::default();
    let instructions = vec![Instruction::STK(Register::None, Register::A)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_pop() {
    let consts = Constants::default();
    let instructions = vec![Instruction::STK(Register::A, Register::None)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_stk_push_pop() {
    let consts = Constants::default();
    let instructions = vec![Instruction::STK(Register::A, Register::B)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_ldm() {
    let consts = Constants::default();
    let instructions = vec![Instruction::LDM(Register::A, Register::B)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_cmp() {
    let consts = Constants::default();
    let instructions = vec![Instruction::CMP(Register::A, Register::B)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_jmp() {
    let consts = Constants::default();
    let instructions = vec![Instruction::JMP("L".try_into().unwrap(), Register::A)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}

#[test]
fn test_sys() {
    let consts = Constants::default();
    let instructions = vec![Instruction::SYS(0x1, Register::D)];
    let bytes = assemble(consts, &instructions);

    assert_eq!(disassemble(consts, bytes).unwrap(), instructions);
}
