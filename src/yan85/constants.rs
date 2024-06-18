#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Constants {
    pub flag: Flag,
    pub opcode: Opcode,
    pub register: Register,
    pub syscall: Syscall,
    pub byte_order: ByteOrder,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Flag {
    pub L: u8,
    pub G: u8,
    pub E: u8,
    pub N: u8,
    pub Z: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Opcode {
    pub IMM: u8,
    pub ADD: u8,
    pub STK: u8,
    pub STM: u8,
    pub LDM: u8,
    pub CMP: u8,
    pub JMP: u8,
    pub SYS: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Register {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub S: u8,
    pub I: u8,
    pub F: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Syscall {
    pub OPEN: u8,
    pub READ_MEMORY: u8,
    pub WRITE: u8,
    pub EXIT: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ByteOrder {
    pub op: u8,
    pub a: u8,
    pub b: u8,
}

pub const TEST_CONSTANTS: Constants = Constants {
    flag: TEST_FLAGS,
    opcode: TEST_OPCODES,
    register: TEST_REGISTERS,
    syscall: TEST_SYSCALLS,
    byte_order: TEST_BYTE_ORDER,
};

const TEST_FLAGS: Flag = Flag {
    L: 0x1,
    G: 0x8,
    E: 0x4,
    N: 0x10,
    Z: 0x2,
};

const TEST_OPCODES: Opcode = Opcode {
    IMM: 0x1,
    ADD: 0x2,
    STK: 0x80,
    STM: 0x10,
    LDM: 0x20,
    CMP: 0x8,
    JMP: 0x40,
    SYS: 0x4,
};

const TEST_REGISTERS: Register = Register {
    A: 0x20,
    B: 0x40,
    C: 0x8,
    D: 0x2,
    S: 0x10,
    I: 0x4,
    F: 0x1,
};

const TEST_SYSCALLS: Syscall = Syscall {
    OPEN: 0x40,
    READ_MEMORY: 0x10,
    WRITE: 0x1,
    EXIT: 0x8,
};

const TEST_BYTE_ORDER: ByteOrder = ByteOrder { op: 2, a: 0, b: 1 };
