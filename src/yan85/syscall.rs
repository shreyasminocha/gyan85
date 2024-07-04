use anyhow::{anyhow, Result};

use super::constants::{Constants, Decodable, Encodable};

/// A Yan85 system call.
pub enum Syscall {
    /// Opens a file.
    Open,
    /// Reads bytes from an open file into Yan85 instructions.
    ReadCode,
    /// Reads bytes from an open file into memory.
    ReadMemory,
    /// Writes bytes from memory to an open file.
    Write,
    /// Pauses execution and sleeps for a finite number of seconds.
    Sleep,
    /// Terminates the program.
    Exit,
}

impl Encodable for Syscall {
    fn encode(&self, c: Constants) -> u8 {
        match self {
            Syscall::Open => c.syscall.OPEN,
            Syscall::ReadCode => c.syscall.READ_CODE,
            Syscall::ReadMemory => c.syscall.READ_MEMORY,
            Syscall::Write => c.syscall.WRITE,
            Syscall::Sleep => c.syscall.SLEEP,
            Syscall::Exit => c.syscall.EXIT,
        }
    }
}

impl Decodable for Syscall {
    fn decode(value: u8, c: Constants) -> Result<Self> {
        match value {
            _ if value == c.syscall.OPEN => Ok(Syscall::Open),
            _ if value == c.syscall.READ_CODE => Ok(Syscall::ReadCode),
            _ if value == c.syscall.READ_MEMORY => Ok(Syscall::ReadMemory),
            _ if value == c.syscall.WRITE => Ok(Syscall::Write),
            _ if value == c.syscall.SLEEP => Ok(Syscall::Sleep),
            _ if value == c.syscall.EXIT => Ok(Syscall::Exit),
            _ => Err(anyhow!("unsupported syscall: {value:#02x}")),
        }
    }
}
