use bevy::prelude::*;

use crate::mods::map::map_position::MapPosition;

#[derive(Component)]
pub struct Chloroplast;

#[derive(Bundle)]
pub struct ChloroplastBundle {
    chloroplast: Chloroplast,
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl ChloroplastBundle {
    pub(super) fn new(
        map_position: MapPosition,
        cell_width: f32,
        start_x: f32,
        start_y: f32,
    ) -> Self {
        ChloroplastBundle {
            chloroplast: Chloroplast,
            map_position,
            draw: SpriteBundle {
                sprite: Sprite {
                    color: Color::LIME_GREEN,
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
