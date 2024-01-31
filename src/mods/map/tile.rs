use bevy::{ecs::bundle::Bundle, math::{Vec2, Vec3}, render::color::Color, sprite::{Sprite, SpriteBundle}, transform::components::Transform};

use super::{map_position::MapPosition, settings::MapSettings};


#[derive(Bundle)]
pub struct TileBundle {
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl TileBundle {
    pub(super) fn new(position: (i32, i32), colour: Color, cell_width: f32, start_x: f32, start_y: f32) -> Self {
        TileBundle {
            map_position: MapPosition {
                x: position.0,
                y: position.1,
            },
            draw: SpriteBundle {
                sprite: Sprite {
                    color: colour,
                    custom_size: Some(Vec2::new(cell_width, cell_width)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(
                    start_x + position.0 as f32 * cell_width,
                    start_y + position.1 as f32 * cell_width,
                    0.,
                )),
                ..Default::default()
            },
        }
    }
}
