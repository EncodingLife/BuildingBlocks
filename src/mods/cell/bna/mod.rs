use super::builder_instructions::*;
use crate::mods::cell::organelle::types::OrganelleType;
use crate::mods::simulation::map::direction::MapDirection;
use std::ops::{Index, IndexMut};

use rand::Rng;

pub mod builder_instructions;

pub const BNA_LENGTH: usize = 32;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct BNA {
    data: [u8; BNA_LENGTH],
}

impl BNA {
    pub fn GREENIE() -> Self {
        let mut data = [0; BNA_LENGTH];

        data[0] = BuilderInstruction::Create(MapDirection::Up, 4).encode();
        data[1] = BuilderInstruction::Create(MapDirection::Right, 3).encode();
        data[2] = BuilderInstruction::Create(MapDirection::Down, 2).encode();
        data[3] = BuilderInstruction::Create(MapDirection::Left, 1).encode();
        data[4] = BuilderInstruction::ReplaceSelf(OrganelleType::Chloroplast).encode();

        Self { data }
    }

    pub const WALKER: Self = {
        let mut data = [0; BNA_LENGTH];
        data[0] = 3;
        Self { data }
    };

    pub const SNAKE: Self = {
        let mut data = [0b00000110; BNA_LENGTH];

        data[0] = 19; // BuildInstruction::Create(MapDirection::Right, 2).encode();
        data[2] = 37; // BuildInstruction::Create(MapDirection::Up, 4).encode();
        data[4] = 51; // BuildInstruction::Create(MapDirection::Right, 6).encode();
        data[6] = 7; //BuildInstruction::Create(MapDirection::Down, 0).encode();

        Self { data }
    };

    pub fn rand() -> Self {
        Self {
            data: [rand::thread_rng().gen(); BNA_LENGTH],
        }
    }

    pub fn get_instruction(&self, index: u8) -> BuilderInstruction {
        if index >= BNA_LENGTH as u8 {
            return BuilderInstruction::default();
        }

        let ins = self[index.into()];

        match ins & 0b1 {
            0b0 => BuilderInstruction::ReplaceSelf(get_cell_type(ins)),
            0b1 => BuilderInstruction::Create(
                get_map_direction(ins),
                index + get_instruction_reference_offset(ins),
            ),
            _ => BuilderInstruction::ReplaceSelf(OrganelleType::None),
        }
    }
}

fn get_cell_type(ins: u8) -> OrganelleType {
    ((ins >> 1) & 0b111).into()
}

fn get_map_direction(ins: u8) -> MapDirection {
    ((ins >> 1) & 0b11).into()
}

fn get_instruction_reference_offset(ins: u8) -> u8 {
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
    use super::*;

    #[test]
    fn empty_bna_without_iref_should_generate_self_destruct() {
        let bna = BNA::default();
        assert_eq!(
            bna.get_instruction(0),
            BuilderInstruction::ReplaceSelf(OrganelleType::None)
        )
    }

    #[test]
    fn bna_with_replace_self_with_chloroplast_as_first_instruction_with_no_start_index() {
        let mut bna = BNA::default();
        bna[0] = BuilderInstruction::ReplaceSelf(OrganelleType::Chloroplast).encode();
        assert_eq!(
            bna.get_instruction(0),
            BuilderInstruction::ReplaceSelf(OrganelleType::Chloroplast)
        )
    }

    #[test]
    fn bna_with_create_as_first_instruction_with_no_start_index() {
        let mut bna = BNA::default();
        bna[0] = BuilderInstruction::Create(MapDirection::Down, 8).encode();
        assert_eq!(
            bna.get_instruction(0),
            BuilderInstruction::Create(MapDirection::Down, 8)
        )
    }
}
