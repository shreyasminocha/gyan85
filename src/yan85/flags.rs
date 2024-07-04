use std::fmt::{self, Display, Write};

use anyhow::{bail, Result};

use super::constants::{Constants, Decodable, Encodable};

/// Comparison flags.
///
/// Used to represent both the results of a comparison (CMP) operation and the conditions necessary
/// of the previous comparison—as stored in the "f" register—for a jump (JMP) operation.
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Flags {
    /// Strictly less than.
    pub less_than: bool,
    /// Strictly greater than.
    pub greater_than: bool,
    /// Equals.
    pub equal: bool,
    /// Does not equal.
    pub not_equal: bool,
    /// "Zero flag" activated when both values equal zero.
    pub zeroes: bool,
}

impl Flags {
    /// Checks if any of the "enabled" flags in `other` are also enabled in `self`.
    pub fn does_match(&self, other: &Flags) -> bool {
        (self.less_than && other.less_than)
            || (self.greater_than && other.greater_than)
            || (self.equal && other.equal)
            || (self.not_equal && other.not_equal)
            || (self.zeroes && other.zeroes)
    }
}

impl Encodable for Flags {
    fn encode(&self, c: Constants) -> u8 {
        let mut value = 0;

        if self.less_than {
            value |= c.flag.L;
        }
        if self.greater_than {
            value |= c.flag.G;
        }
        if self.equal {
            value |= c.flag.E;
        }
        if self.not_equal {
            value |= c.flag.N;
        }
        if self.zeroes {
            value |= c.flag.Z;
        }

        value
    }
}

impl Decodable for Flags {
    fn decode(value: u8, c: Constants) -> Result<Self> {
        let mut flags = Flags::default();

        if value & c.flag.L != 0 {
            flags.less_than = true;
        }
        if value & c.flag.G != 0 {
            flags.greater_than = true;
        }
        if value & c.flag.E != 0 {
            flags.equal = true;
        }
        if value & c.flag.N != 0 {
            flags.not_equal = true;
        }
        if value & c.flag.Z != 0 {
            flags.zeroes = true;
        }

        Ok(flags)
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.less_than {
            f.write_char('L')?;
        }
        if self.greater_than {
            f.write_char('G')?;
        }
        if self.equal {
            f.write_char('E')?;
        }
        if self.not_equal {
            f.write_char('N')?;
        }
        if self.zeroes {
            f.write_char('Z')?;
        }

        fmt::Result::Ok(())
    }
}

impl TryFrom<&str> for Flags {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut flags = Flags::default();

        for c in value.chars() {
            match c {
                'L' => {
                    flags.less_than = true;
                }
                'G' => {
                    flags.greater_than = true;
                }
                'E' => {
                    flags.equal = true;
                }
                'N' => {
                    flags.not_equal = true;
                }
                'Z' => {
                    flags.zeroes = true;
                }
                _ => bail!("invalid character in flags: '{c}'"),
            };
        }

        Ok(flags)
    }
}
