use bevy::prelude::*;

use super::{
    compound_storage::VolatileEnergyStorage,
    maintenance::{Maintenance, MaintenanceBundle},
};

#[derive(Bundle)]
pub struct OrganelleBundle<T>
where
    T: Bundle,
{
    maintenance_bundle: MaintenanceBundle,
    organelle_bundle: T,
}

impl<T> OrganelleBundle<T>
where
    T: Bundle,
{
    pub fn new(o_bundle: T, maintenance_tick_rate: u16, starting_energy: u16) -> Self {
        Self {
            organelle_bundle: o_bundle,
            maintenance_bundle: MaintenanceBundle {
                maintenance: Maintenance {
                    tick_rate: maintenance_tick_rate,
                    remaining_ticks: maintenance_tick_rate,
                },
                volatile_energy_storage: VolatileEnergyStorage(starting_energy),
            },
        }
    }
}
