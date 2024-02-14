use bevy::prelude::*;

use self::{
    behaviours::{cell_death::cell_death_due_to_no_organelle, cell_spawn::cell_spawned},
    bna::*,
    organelle::OrganellePlugin,
};

use super::simulation::map::position::MapPosition;

mod behaviours;
pub mod bna;
pub mod cell_bundle;
pub mod organelle;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellSpawned>()
            .add_plugins(OrganellePlugin)
            .add_systems(Update, (cell_death_due_to_no_organelle, cell_spawned));
    }
}

#[derive(Component)]
pub struct Cell {
    pub bna: BNA,
}

#[derive(Event)]
pub struct CellSpawned(pub BNA, pub MapPosition);
