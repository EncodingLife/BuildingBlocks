use bevy::prelude::*;

use super::{cell::{bna::BNA, CellSpawned}, simulation::map::position::MapPosition};

pub fn debug_setup(mut ev_writer: EventWriter<CellSpawned>) {
    ev_writer.send(CellSpawned(BNA::GREENIE(), MapPosition::new(5,5)));
}
