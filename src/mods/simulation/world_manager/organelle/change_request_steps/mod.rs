use crate::mods::{cell::organelle::organelle_structural_change_request::OrganelleStructuralChangeRequest, simulation::map::collision::CollisionMap};
use bevy::prelude::*;

use self::position_is_empty::PositionIsEmpty;

mod position_is_empty;
mod has_sufficient_volatile_energy;

pub struct OrganelleChangeRequestContext<'a> {
    pub map: &'a CollisionMap
}

pub trait OrganelleChangeRequestApprovalHandler {
    fn set_next(&mut self, next: Box<dyn OrganelleChangeRequestApprovalHandler>);
    fn handle(&self, change: &OrganelleStructuralChangeRequest, context: &OrganelleChangeRequestContext) -> bool;
}

pub fn get_pipeline() -> Box<dyn OrganelleChangeRequestApprovalHandler> {
    Box::new(PositionIsEmpty { next: None })
}