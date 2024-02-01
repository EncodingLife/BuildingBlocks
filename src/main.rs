use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use mods::{
    cell::CellPlugin, interface::InterfacePlugin, map::MapPlugin, organism::{Organism, OrganismPlugin}, tick::TickPlugin
};

use crate::mods::{cell::{instruction::instruction::Instruction, r#type::CellType}, map::direction::MapDirection};

mod mods;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MapPlugin,
            TickPlugin,
            CellPlugin,
            OrganismPlugin,
            InterfacePlugin
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    println!("{}", Instruction::Create(MapDirection::Right, 0).encode());
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
