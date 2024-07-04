use std::ops::{Index, IndexMut, Range, RangeFrom};

use anyhow::bail;

/// The size of the code storage in bytes.
///
/// Can store up to 256 instructions, each occupying 3 bytes.
const CODE_SIZE: usize = 256 * 3;

/// 768-byte Yan85 code storage.
#[derive(Debug)]
pub struct Code([u8; CODE_SIZE]);

impl Code {
    /// Get the instruction 3-tuple for instruction number `number`.
    pub fn get_instruction(&self, number: u8) -> [u8; 3] {
        [
            self[3 * number as usize],
            self[((3 * number) + 1) as usize],
            self[((3 * number) + 2) as usize],
        ]
    }
}

impl Default for Code {
    fn default() -> Self {
        Self([0; CODE_SIZE])
    }
}

impl From<[u8; CODE_SIZE]> for Code {
    fn from(value: [u8; CODE_SIZE]) -> Self {
        Self(value)
    }
}

impl TryFrom<Vec<u8>> for Code {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut code = value;

        if code.len() > CODE_SIZE {
            bail!("The emulator can't fit more than 256 instructions");
        }

        code.resize(CODE_SIZE, 0);

        Ok(Self(
            code.try_into().expect("We resized it to the correct size"),
        ))
    }
}

impl Index<usize> for Code {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<Range<usize>> for Code {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index.start..index.end]
    }
}

impl Index<RangeFrom<usize>> for Code {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index.start..]
    }
}

impl IndexMut<usize> for Code {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IndexMut<Range<usize>> for Code {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index.start..index.end]
    }
}
