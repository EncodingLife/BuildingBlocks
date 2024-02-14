use crate::mods::{cell::organelle::sustenance::Sustenance, simulation::tick::Ticked};
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
mod utilizable_energy;

pub trait OrganelleStructure {
    const COLOUR: Color;
    const STARTING_UTILIZABLE_ENERGY: u16;
}

pub trait OrganelleFunctions {
    fn get_sustenance() -> Sustenance;
}

pub struct OrganellePlugin;

impl Plugin for OrganellePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrganelleStructuralChangeRequest>()
            .register_type::<Sustenance>()
            .register_type::<UtilizableEnergy>()
            .add_systems(
                Update,
                (
                    consume_energy_for_maintenance,
                    queue_nucleus_structural_change_requests,
                    queue_builder_structural_change_requests,
                )
                    .run_if(on_event::<Ticked>()),
            );
    }
}
