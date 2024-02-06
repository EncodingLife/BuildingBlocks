use bevy::prelude::*;

use crate::mods::map::map_position::MapPosition;

pub mod instruction_execution;

#[derive(Component, Debug)]
pub struct Stem {
    current_instruction: u8,
}

impl Stem {
    fn new(current_instruction: u8) -> Self {
        Self {
            current_instruction,
        }
    }
}

#[derive(Bundle)]
pub struct StemBundle {
    stem: Stem,
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl StemBundle {
    pub(super) fn new(
        map_position: MapPosition,
        cell_width: f32,
        start_x: f32,
        start_y: f32,
        iref: Option<u8>,
    ) -> Self {
        StemBundle {
            stem: Stem::new(iref.unwrap_or_default()),
            map_position,
            draw: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
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
