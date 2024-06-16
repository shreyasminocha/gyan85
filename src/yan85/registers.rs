use std::ops::{Index, IndexMut};

use crate::yan85::register::Register;

#[derive(Default)]
pub struct Registers([u8; 7]);

impl Index<Register> for Registers {
    type Output = u8;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
