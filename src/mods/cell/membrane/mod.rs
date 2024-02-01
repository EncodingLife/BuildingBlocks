use bevy::prelude::*;

use crate::mods::map::map_position::MapPosition;

#[derive(Component)]
pub struct Membrane;

#[derive(Bundle)]
pub struct MembraneBundle {
    membrane: Membrane,
    map_position: MapPosition,
    draw: SpriteBundle,
}

impl MembraneBundle {
    pub(super) fn new(
        map_position: MapPosition,
        cell_width: f32,
        start_x: f32,
        start_y: f32,
    ) -> Self {
        MembraneBundle {
            membrane: Membrane,
            map_position,
            draw: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
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
