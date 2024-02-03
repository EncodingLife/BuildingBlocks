use std::ops::{Index, IndexMut};


use rand::Rng;

use crate::mods::map::direction::MapDirection;

use super::{
    instruction::{instruction::Instruction},
    r#type::CellType,
};

pub const BNA_LENGTH: usize = 32;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct BNA {
    data: [u8; BNA_LENGTH],
}

impl BNA {
    pub const GREENIE: Self = Self{data:[6;BNA_LENGTH]};
    pub const WALKER: Self = {
        let mut data = [0;BNA_LENGTH];
        data[0] = 3;
        Self { data }
    };

    pub fn rand() -> Self {
        Self {
            data: [rand::thread_rng().gen(); BNA_LENGTH],
        }
    }

    pub fn get_instruction(&self, index: u8) -> Instruction {
        let ins = self[index.into()];
        match ins & 0b1 {
            0b0 => Instruction::ReplaceSelf(get_cell_type(ins)),
            0b1 => Instruction::Create(get_map_direction(ins), get_instruction_reference(ins)),
            _ => Instruction::ReplaceSelf(CellType::None),
        }
    }
}

fn get_cell_type(ins: u8) -> CellType {
    ((ins >> 1) & 0b111).into()
}

fn get_map_direction(ins: u8) -> MapDirection {
    ((ins >> 1) & 0b11).into()
}

fn get_instruction_reference(ins: u8) -> u8 {
    ((ins >> 3) & 0b11111).into()
}

impl IndexMut<usize> for BNA {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<usize> for BNA {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;

    use bevy::render::render_resource::encase::rts_array::Length;

    use crate::mods::cell::r#type::CellType;

    use super::*;

    #[test]
    fn empty_bna_without_iref_should_generate_self_destruct() {
        let bna = BNA::default();
        assert_eq!(
            bna.get_instruction(0),
            Instruction::ReplaceSelf(CellType::None)
        )
    }

    #[test]
    fn bna_with_replace_self_with_chloroplast_as_first_instruction_with_no_start_index() {
        let mut bna = BNA::default();
        bna[0] = Instruction::ReplaceSelf(CellType::Chloroplast).encode();
        assert_eq!(
            bna.get_instruction(0),
            Instruction::ReplaceSelf(CellType::Chloroplast)
        )
    }

    #[test]
    fn bna_with_create_as_first_instruction_with_no_start_index() {
        let mut bna = BNA::default();
        bna[0] = Instruction::Create(MapDirection::Down, 8).encode();
        assert_eq!(
            bna.get_instruction(0),
            Instruction::Create(MapDirection::Down, 8)
        )
    }
}
