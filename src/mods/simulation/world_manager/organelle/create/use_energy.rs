use bevy::prelude::*;

use crate::mods::cell::organelle::{types::builder::Builder, utilizable_energy::UtilizableEnergy};

use super::OrganelleCreated;

pub fn use_energy(
    mut ev_reader: EventReader<OrganelleCreated>,
    mut query: Query<&mut UtilizableEnergy>,
) {
    for &OrganelleCreated(_, _, _, _, s) in ev_reader.read() {
        if let Some(source) = s {
            match query.get_mut(source) {
                Ok(mut ue) => {
                    ue.0 = ue.0.checked_sub(Builder::STRUCTURE.spawn_energy_cost + Builder::STRUCTURE.starting_energy).unwrap_or_default();
                }
                Err(err) => debug!("use_energy {err}"),
            }
        }
    }
}
