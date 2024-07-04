use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Instruction encoding specification that varies from level to level.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constants {
    /// The byte order of the instruction 3-tuple.
    pub byte_order: ByteOrderConstants,
    /// The opcode constants.
    pub opcode: OpcodeConstants,
    /// The register constants.
    pub register: RegisterConstants,
    /// The syscall numbers.
    pub syscall: SyscallConstants,
    /// The flag constants.
    pub flag: FlagConstants,
}

/// Encodable to a byte with the constants dictionary as context.
pub trait Encodable {
    /// Encodes the struct to a byte.
    fn encode(&self, constants: Constants) -> u8;
}

/// Decodable from a byte with the constants dictionary as context.
pub trait Decodable {
    /// Decodes the struct from a byte.
    fn decode(value: u8, constants: Constants) -> Result<Self>
    where
        Self: std::marker::Sized;
}

/// Specification of the encoding order of the instruction 3-tuple.
///
/// The values of this structure's fields must be a permutation of {0, 1, 2}.
///
/// # Examples
///
/// ```
/// use gyan85::yan85::constants::ByteOrderConstants;
///
/// let byte_order = ByteOrderConstants { op: 0, a: 1, b: 2 };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteOrderConstants {
    /// Index of the byte corresponding to the opcode.
    pub op: u8,
    /// Index of the byte corresponding to the first operand.
    pub a: u8,
    /// Index of the byte corresponding to the second operand.
    pub b: u8,
}

impl Default for ByteOrderConstants {
    fn default() -> Self {
        Self { op: 0, a: 1, b: 2 }
    }
}

/// The constants associated with each opcode.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpcodeConstants {
    /// The constant for the IMM opcode.
    pub IMM: u8,
    /// The constant for the ADD opcode.
    pub ADD: u8,
    /// The constant for the STK opcode.
    pub STK: u8,
    /// The constant for the STM opcode.
    pub STM: u8,
    /// The constant for the LDM opcode.
    pub LDM: u8,
    /// The constant for the CMP opcode.
    pub CMP: u8,
    /// The constant for the JMP opcode.
    pub JMP: u8,
    /// The constant for the SYS opcode.
    pub SYS: u8,
}

impl Default for OpcodeConstants {
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

/// The constants associated with each register.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterConstants {
    /// The constant for the "a" register.
    pub A: u8,
    /// The constant for the "b" register.
    pub B: u8,
    /// The constant for the "c" register.
    pub C: u8,
    /// The constant for the "d" register.
    pub D: u8,
    /// The constant for the "s" (stack pointer) register.
    pub S: u8,
    /// The constant for the "i" (instruction pointer) register.
    pub I: u8,
    /// The constant for the "f" (flag) register.
    pub F: u8,
}

impl Default for RegisterConstants {
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

/// The syscall numbers associated with each syscall.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyscallConstants {
    /// The syscall number for the `OPEN` syscall.
    pub OPEN: u8,
    /// The syscall number for the `READ_CODE` syscall.
    pub READ_CODE: u8,
    /// The syscall number for the `READ_MEMORY` syscall.
    pub READ_MEMORY: u8,
    /// The syscall number for the `WRITE` syscall.
    pub WRITE: u8,
    /// The syscall number for the `SLEEP` syscall.
    pub SLEEP: u8,
    /// The syscall number for the `EXIT` syscall.
    pub EXIT: u8,
}

impl Default for SyscallConstants {
    fn default() -> Self {
        Self {
            OPEN: 0x1,
            READ_CODE: 0x2,
            READ_MEMORY: 0x4,
            WRITE: 0x8,
            SLEEP: 0x10,
            EXIT: 0x20,
        }
    }
}

/// The constants associated with each flag.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlagConstants {
    /// The constant for the "less than" flag.
    pub L: u8,
    /// The constant for the "greater than" flag.
    pub G: u8,
    /// The constant for the "equal to" flag.
    pub E: u8,
    /// The constant for the "not equal to" flag.
    pub N: u8,
    /// The constant for the "zeroes" flag.
    pub Z: u8,
}

impl Default for FlagConstants {
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
