use crate::mods::{
    cell::organelle::sustenance::Sustenance,
    simulation::{tick::Ticked, FixedBehaviour, TriggerBehaviour},
};
use bevy::prelude::*;

use self::{
    behaviours::{
        consume_energy_for_maintenance::consume_energy_for_maintenance,
        queue_builder_change_requests::*, queue_nucleus_change_requests::*,
    },
    organelle_structural_change_request::OrganelleStructuralChangeRequest,
    utilizable_energy::UtilizableEnergy,
};

mod behaviours;
pub mod organelle_bundle;
pub mod organelle_structural_change_request;
mod sustenance;
pub mod types;
pub mod utilizable_energy;

pub struct OrganelleStructure {
    pub color: Color,
    pub starting_energy: u16,
    pub spawn_energy_cost: u16,
}

pub trait OrganelleFunctions {
    fn get_sustenance() -> Sustenance;
    fn get_structure() -> OrganelleStructure;
}

pub struct OrganellePlugin;

impl Plugin for OrganellePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrganelleStructuralChangeRequest>()
            .register_type::<Sustenance>()
            .register_type::<UtilizableEnergy>()
            .add_systems(
                FixedBehaviour,
                (
                    consume_energy_for_maintenance,
                    queue_nucleus_structural_change_requests,
                ),
            )
            .add_systems(
                TriggerBehaviour,
                (queue_builder_structural_change_requests,),
            );
    }
}
