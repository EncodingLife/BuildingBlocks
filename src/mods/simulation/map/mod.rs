use bevy::prelude::*;

use collision::*;

use self::position::MapPosition;

pub mod collision;
pub mod direction;
pub mod position;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<MapPosition>()
            .insert_resource(CollisionMap::new());
    }
}
