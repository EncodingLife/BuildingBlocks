use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default)]
pub struct BNA {
    data: [u16; 16],
}

impl IndexMut<usize> for BNA {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<usize> for BNA {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
