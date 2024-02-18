use bevy::prelude::*;

use crate::mods::{cell::organelle::types::OrganelleType, simulation::map::position::MapPosition};

pub(super) mod create_organelle;
pub(super) mod use_energy;

#[derive(Event)]
pub struct OrganelleCreated(
    pub MapPosition,
    pub u8,
    pub Entity,
    pub OrganelleType,
    pub Option<Entity>,
);
