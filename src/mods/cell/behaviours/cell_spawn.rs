use bevy::prelude::*;

use crate::mods::cell::{
    cell_bundle::CellBundle,
    organelle::{organelle_bundle::OrganelleBundle, types::nucleus::Nucleus},
    CellSpawned,
};

pub fn cell_spawned(mut command: Commands, mut ev_reader: EventReader<CellSpawned>) {
    for &CellSpawned(bna, mp) in ev_reader.read() {
        println!("Spawning new entity at {mp:?}");
        command.spawn(CellBundle::new(bna)).with_children(|p| {
            p.spawn(OrganelleBundle::new(Nucleus, mp));
        });
    }
}
