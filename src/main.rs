use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use mods::{
    cell::CellPlugin,
    interface::InterfacePlugin,
    map::MapPlugin,
    organism::{OrganismPlugin},
    shared::{simulation_settings::{WINDOW_HEIGHT, WINDOW_WIDTH}, SharedPlugin},
    tick::TickPlugin,
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
            MapPlugin,
            TickPlugin,
            CellPlugin,
            OrganismPlugin,
            InterfacePlugin,
            SharedPlugin
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}
