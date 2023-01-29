pub mod flag {
    pub const L: u8 = 0x1;
    pub const G: u8 = 0x8;
    pub const E: u8 = 0x4;
    pub const N: u8 = 0x10;
    pub const Z: u8 = 0x2;
}

pub mod opcode {
    pub const IMM: u8 = 0x1;
    pub const ADD: u8 = 0x2;
    pub const STK: u8 = 0x80;
    pub const STM: u8 = 0x10;
    pub const LDM: u8 = 0x20;
    pub const CMP: u8 = 0x8;
    pub const JMP: u8 = 0x40;
    pub const SYS: u8 = 0x4;
}

pub mod register {
    pub const A: u8 = 0x20;
    pub const B: u8 = 0x40;
    pub const C: u8 = 0x8;
    pub const D: u8 = 0x2;
    pub const S: u8 = 0x10;
    pub const I: u8 = 0x4;
    pub const F: u8 = 0x1;
}

pub mod syscall {
    pub const READ_MEMORY: u8 = 0x10;
    pub const WRITE: u8 = 0x1;
    pub const EXIT: u8 = 0x8;
}
