use bevy::prelude::*;

use self::{
    fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter},
    simulation_info::SimulationInfoPlugin,
};

mod fps;
mod map_draw;
mod simulation_info;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimulationInfoPlugin)
            .add_systems(Startup, (setup_fps_counter, setup_camera))
            .add_systems(Update, (fps_text_update_system, fps_counter_showhide));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
