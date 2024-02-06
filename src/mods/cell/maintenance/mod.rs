use bevy::prelude::*;

use crate::mods::{map::map_position::MapPosition, organism::Organism};

use self::organelle_died::OrganelleDied;

use super::compound_storage::VolatileEnergyStorage;

pub mod organelle_died;

#[derive(Component, Debug, Copy, Clone)]
pub struct Maintenance {
    pub tick_rate: u16,
    pub remaining_ticks: u16,
}
pub(super) fn consume_energy_for_maintenance(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Parent,
        &mut Maintenance,
        &mut VolatileEnergyStorage,
    )>,
    mut parent_query: Query<Entity, With<Organism>>,
) {
    for (e, parent, mut m, mut ves) in query.iter_mut() {
        if m.remaining_ticks == 0 {
            m.remaining_ticks = m.tick_rate;
            if ves.0 == 0 {
                commands.entity(parent.get()).remove_children(&[e]);
                commands.entity(e).despawn();
                commands.entity(parent.get()).insert(OrganelleDied);
                println!("Despawning organelle {e:?}");
            } else {
                ves.0 = ves.0 - 1;
            }
        } else {
            m.remaining_ticks = m.remaining_ticks - 1;
        }
    }
}

#[derive(Bundle)]
pub struct MaintenanceBundle {
    pub maintenance: Maintenance,
    pub volatile_energy_storage: VolatileEnergyStorage,
}
