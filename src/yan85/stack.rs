use std::ops::{Index, IndexMut, Range};

pub struct Stack([u8; 256]);

impl Default for Stack {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl Index<u8> for Stack {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Index<Range<u8>> for Stack {
    type Output = [u8];

    fn index(&self, index: Range<u8>) -> &Self::Output {
        &self.0[index.start as usize..index.end as usize]
    }
}

impl IndexMut<u8> for Stack {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl IndexMut<Range<u8>> for Stack {
    fn index_mut(&mut self, index: Range<u8>) -> &mut Self::Output {
        &mut self.0[index.start as usize..index.end as usize]
    }
}
