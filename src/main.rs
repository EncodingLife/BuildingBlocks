use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use mods::{
    cell::CellPlugin,
    interface::InterfacePlugin,
    map::MapPlugin,
    organism::{Organism, OrganismPlugin},
    shared::{simulation_settings::{WINDOW_HEIGHT, WINDOW_WIDTH}, SharedPlugin},
    tick::TickPlugin,
};

use crate::mods::{
    cell::{instruction::instruction::Instruction, r#type::CellType},
    map::direction::MapDirection,
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
