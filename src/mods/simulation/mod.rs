use bevy::prelude::*;

use self::{map::MapPlugin, tick::TickPlugin, world_manager::WorldManagerPlugin};

pub mod elements;
pub mod map;
pub mod settings;
pub mod tick;
pub mod world_manager;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TickPlugin, WorldManagerPlugin, MapPlugin));
    }
}
