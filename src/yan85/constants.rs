use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constants {
    pub flag: Flag,
    pub opcode: Opcode,
    pub register: Register,
    pub syscall: Syscall,
    pub byte_order: ByteOrder,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flag {
    pub L: u8,
    pub G: u8,
    pub E: u8,
    pub N: u8,
    pub Z: u8,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            L: 0x1,
            G: 0x2,
            E: 0x4,
            N: 0x8,
            Z: 0x10,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for Opcode {
    fn default() -> Self {
        Self {
            IMM: 0x1,
            ADD: 0x2,
            STK: 0x4,
            STM: 0x8,
            LDM: 0x10,
            CMP: 0x20,
            JMP: 0x40,
            SYS: 0x80,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Register {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub S: u8,
    pub I: u8,
    pub F: u8,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            A: 0x1,
            B: 0x2,
            C: 0x4,
            D: 0x8,
            S: 0x10,
            I: 0x20,
            F: 0x40,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Syscall {
    pub OPEN: u8,
    pub READ_MEMORY: u8,
    pub WRITE: u8,
    pub EXIT: u8,
}

impl Default for Syscall {
    fn default() -> Self {
        Self {
            OPEN: 0x1,
            READ_MEMORY: 0x2,
            WRITE: 0x4,
            EXIT: 0x8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteOrder {
    pub op: u8,
    pub a: u8,
    pub b: u8,
}

impl Default for ByteOrder {
    fn default() -> Self {
        Self { op: 0, a: 1, b: 2 }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_example_constants_yml() {
        let yaml = fs::read_to_string("constants.yml").unwrap();
        let consts: Constants = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(consts, Constants::default())
    }
}
