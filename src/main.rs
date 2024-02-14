use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use mods::{
    cell::CellPlugin, debug_setup::*, interface::InterfacePlugin, simulation::{
        settings::{WINDOW_HEIGHT, WINDOW_WIDTH},
        SimulationPlugin,
    }
};

mod mods;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            CellPlugin,
            InterfacePlugin,
            SimulationPlugin,
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, debug_setup)
        .run();
}
