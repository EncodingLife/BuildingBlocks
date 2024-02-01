use bevy::prelude::*;

use self::{collision_map::CollisionMap, settings::MapSettings};

pub mod map_position;
pub mod settings;
pub mod direction;
pub mod collision_map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MapSettings::default());
        app.insert_resource(CollisionMap::new());
    }
}