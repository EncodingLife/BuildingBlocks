use bevy::prelude::*;

use crate::mods::{cell::organelle::types::OrganelleType, simulation::map::position::MapPosition};

mod create_organelle;
mod use_energy;

pub use create_organelle::create_organelle;
pub use use_energy::use_energy;

#[derive(Event)]
pub struct OrganelleCreated(
    pub MapPosition,
    pub u8,
    pub Entity,
    pub OrganelleType,
    pub Option<Entity>,
);
