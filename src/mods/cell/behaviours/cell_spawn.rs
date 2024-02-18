use bevy::prelude::*;

use crate::mods::{cell::{
    cell_bundle::CellBundle,
    organelle::
        types::OrganelleType
    ,
    CellSpawned,
}, simulation::{settings::MAP_CELL_HEIGHT, world_manager::organelle::create::*}};

pub fn cell_spawned(
    mut command: Commands,
    mut ev_reader: EventReader<CellSpawned>,
    mut ev_create_o: EventWriter<OrganelleCreated>,
) {
    for &CellSpawned(bna, mp) in ev_reader.read() {
        let e = command.spawn(CellBundle::new(bna)).id();

        assert!(mp.y < MAP_CELL_HEIGHT.into());

        // TODO: This feels wrong and should be done without exposing OrganelleCreated

        ev_create_o.send(OrganelleCreated(mp, 0, e, OrganelleType::Nucleus, None));
    }
}
