use bevy::prelude::*;

use self::{tile::TileBundle, settings::MapSettings};

mod tile;
pub mod map_position;
pub mod settings;
pub mod direction;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MapSettings::default());
    }
}

fn dummy_setup(
    map_settings: Res<MapSettings>,
    mut commands: Commands,
) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;

    println!("{start_x},{start_y}");
    for x in 0..map_settings.width {
        for y in 0..map_settings.width {
            let c = match ((x + y).wrapping_mul(x.wrapping_sub(y).wrapping_shl(y))) % 3 {
                0 => Color::BLUE,
                1 => Color::PURPLE,
                2 => Color::WHITE,
                3 => Color::CRIMSON,
                4 => Color::ORANGE,
                _ => panic!("woops: {x}+{y}%3={}", (x + y) % 3),
            };
            commands.spawn(TileBundle::new((x as i32, y as i32), c, map_settings.cell_width, start_x, start_y));
        }
    }
}
