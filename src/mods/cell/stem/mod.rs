use bevy::prelude::*;

use crate::mods::map::{direction::MapDirection, map_position::MapPosition};

use super::{
    instruction::{encoded::decode_bna, instruction::Instruction, set::InstructionSet},
    membrane::Membrane,
    r#type::CellType,
};

pub mod instruction_execution;

#[derive(Component)]
pub struct Stem {
    instruction_set: InstructionSet,
}

impl Stem {
    fn new(mut instructions: Vec<Instruction>) -> Self {
        instructions.reverse();
        Self {
            instruction_set: InstructionSet::new(instructions),
        }
    }
}

#[derive(Bundle)]
pub struct StemBundle {
    nucleus: Stem,
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl StemBundle {
    pub(super) fn new(
        map_position: MapPosition,
        cell_width: f32,
        start_x: f32,
        start_y: f32,
        bna: u64
    ) -> Self {
        let instructions = decode_bna(bna);
        StemBundle {
            nucleus: Stem::new(instructions),
            map_position,
            draw: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(cell_width, cell_width)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(
                    start_x + map_position.x as f32 * cell_width,
                    start_y + map_position.y as f32 * cell_width,
                    0.,
                )),
                ..Default::default()
            },
        }
    }
}
