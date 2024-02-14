use bevy::prelude::*;

use crate::mods::{
    cell::builder_instructions::BuilderInstruction, simulation::map::position::MapPosition,
};

#[derive(Event, Copy, Clone, Debug, PartialEq)]
pub struct OrganelleStructuralChangeRequest {
    pub instruction: BuilderInstruction,
    pub source: Entity,
    pub parent: Entity,
    pub target_pos: MapPosition,
}
