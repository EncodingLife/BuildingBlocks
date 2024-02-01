use bevy::prelude::*;

use crate::mods::map::{direction::MapDirection, map_position::MapPosition};

use super::{
    bna::BNA,
    instruction::{instruction::Instruction, set::InstructionSet},
    membrane::Membrane,
    r#type::CellType,
};

#[derive(Component)]
pub struct Nucleus;

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
        start_y: f32
    ) -> Self {
        NucleusBundle {
            nucleus: Nucleus,
            map_position,
            draw: SpriteBundle {
                sprite: Sprite {
                    color: Color::PURPLE,
                    custom_size: Some(Vec2::new(cell_width, cell_width)),
                    ..Default::default()
                },
                global_transform: GlobalTransform::from_translation(Vec3::new(
                    start_x + map_position.x as f32 * cell_width,
                    start_y + map_position.y as f32 * cell_width,
                    0.,
                )),
                ..Default::default()
            },
        }
    }
}