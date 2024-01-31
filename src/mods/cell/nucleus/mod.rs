use bevy::prelude::*;

use crate::mods::map::{direction::MapDirection, map_position::MapPosition};

use super::{
    instruction::{instruction::Instruction, set::InstructionSet},
    membrane::Membrane,
    r#type::CellType,
};

pub mod instruction_execution;

#[derive(Component)]
pub struct Nucleus {
    instruction_set: InstructionSet,
}

impl Nucleus {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instruction_set: InstructionSet::new(instructions),
        }
    }
}

#[derive(Bundle)]
pub struct NucleusBundle {
    nucleus: Nucleus,
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl NucleusBundle {
    pub(super) fn new(
        map_position: MapPosition,
        cell_width: f32,
        start_x: f32,
        start_y: f32,
    ) -> Self {
        let mut i: Vec<_> = vec![
            Instruction::Create(CellType::Membrane, MapDirection::Up),
            Instruction::Create(CellType::Membrane, MapDirection::Right),
            Instruction::Create(CellType::Membrane, MapDirection::Left),
            Instruction::Create(CellType::Membrane, MapDirection::Down),
            Instruction::ReplaceSelf(CellType::Chloroplast),
        ];
        i.reverse();
        NucleusBundle {
            nucleus: Nucleus::new(i),
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
