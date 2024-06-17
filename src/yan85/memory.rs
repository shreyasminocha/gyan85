use std::ops::{Index, IndexMut, Range};

#[derive(Debug)]
pub struct Memory([u8; 256]);

impl Default for Memory {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl From<[u8; 256]> for Memory {
    fn from(value: [u8; 256]) -> Self {
        Self(value)
    }
}

impl Index<u8> for Memory {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Index<Range<u8>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<u8>) -> &Self::Output {
        &self.0[index.start as usize..index.end as usize]
    }
}

impl IndexMut<u8> for Memory {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl IndexMut<Range<u8>> for Memory {
    fn index_mut(&mut self, index: Range<u8>) -> &mut Self::Output {
        &mut self.0[index.start as usize..index.end as usize]
    }
}
