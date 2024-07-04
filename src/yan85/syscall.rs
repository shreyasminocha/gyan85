use std::ops::BitOr;

use anyhow::Result;

use super::constants::{Constants, Decodable, Encodable};

/// A Yan85 system call.
#[derive(Debug, PartialEq, Eq)]
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

impl Encodable for Vec<Syscall> {
    fn encode(&self, c: Constants) -> u8 {
        self.iter()
            .map(|syscall| syscall.encode(c))
            .reduce(|acc, s| acc.bitor(s))
            .unwrap_or(0)
    }
}

impl Decodable for Vec<Syscall> {
    fn decode(value: u8, c: Constants) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut syscalls = vec![];

        if value & c.syscall.OPEN != 0 {
            syscalls.push(Syscall::Open);
        }
        if value & c.syscall.READ_CODE != 0 {
            syscalls.push(Syscall::ReadCode);
        }
        if value & c.syscall.READ_MEMORY != 0 {
            syscalls.push(Syscall::ReadMemory);
        }
        if value & c.syscall.WRITE != 0 {
            syscalls.push(Syscall::Write);
        }
        if value & c.syscall.SLEEP != 0 {
            syscalls.push(Syscall::Sleep);
        }
        if value & c.syscall.EXIT != 0 {
            syscalls.push(Syscall::Exit);
        }

        Ok(syscalls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_one_syscall() {
        let consts = Constants::default();
        assert_eq!(
            Vec::<Syscall>::decode(consts.syscall.READ_MEMORY, consts).unwrap(),
            vec![Syscall::ReadMemory]
        );
    }

    #[test]
    fn test_decode_multiple_syscalls() {
        let consts = Constants::default();
        assert_eq!(
            Vec::<Syscall>::decode(
                consts.syscall.OPEN
                    | consts.syscall.READ_MEMORY
                    | consts.syscall.WRITE
                    | consts.syscall.EXIT,
                consts
            )
            .unwrap(),
            vec![
                Syscall::Open,
                Syscall::ReadMemory,
                Syscall::Write,
                Syscall::Exit
            ]
        );
    }
}
