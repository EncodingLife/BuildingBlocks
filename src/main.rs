use bevy::prelude::*;
use mods::{cell::CellPlugin, map::MapPlugin, tick::TickPlugin};

mod mods;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin, TickPlugin, CellPlugin))
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands,) {
    commands.spawn(Camera2dBundle::default());
}