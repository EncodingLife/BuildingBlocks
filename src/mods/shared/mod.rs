use bevy::app::Plugin;

use self::simulation_settings::SimulationSettings;

pub mod simulation_settings;

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SimulationSettings::new());
    }
}