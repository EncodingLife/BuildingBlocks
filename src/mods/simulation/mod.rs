use bevy::prelude::*;

use self::{
    map::MapPlugin, schedule::SchedulePlugin, tick::TickPlugin, world_manager::WorldManagerPlugin,
};

pub mod elements;
pub mod map;
pub mod schedule;
pub mod settings;
pub mod tick;
pub mod world_manager;

pub use self::schedule::{PreSim,ApproveBehaviour, ExecuteBehaviour, FixedBehaviour, TriggerBehaviour};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TickPlugin, WorldManagerPlugin, MapPlugin));
    }
}
