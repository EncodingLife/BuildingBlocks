use bevy::prelude::*;

use self::{
    behaviours::{cell_death::cell_death_due_to_no_organelle, cell_spawn::cell_spawned},
    bna::*,
    organelle::OrganellePlugin,
};

use super::simulation::{map::position::MapPosition, FixedBehaviour};

mod behaviours;
pub mod bna;
pub mod cell_bundle;
pub mod organelle;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        println!("Cell plugin");
        app.add_event::<CellSpawned>()
            .add_plugins(OrganellePlugin)
            .add_systems(
                FixedBehaviour,
                (cell_spawned, cell_death_due_to_no_organelle),
            );
    }
}

#[derive(Component)]
pub struct Cell {
    pub bna: BNA,
}

#[derive(Event)]
pub struct CellSpawned(pub BNA, pub MapPosition);