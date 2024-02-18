use crate::mods::cell::{
    bna::builder_instructions::BuilderInstruction,
    organelle::organelle_structural_change_request::OrganelleStructuralChangeRequest,
};

use super::{OrganelleChangeRequestApprovalHandler, OrganelleChangeRequestContext};

pub struct PositionIsEmpty {
    pub next: Option<Box<dyn OrganelleChangeRequestApprovalHandler>>,
}

impl OrganelleChangeRequestApprovalHandler for PositionIsEmpty {
    fn set_next(&mut self, next: Box<dyn OrganelleChangeRequestApprovalHandler>) {
        self.next = Some(next);
    }

    fn handle(
        &self,
        change: &OrganelleStructuralChangeRequest,
        context: &OrganelleChangeRequestContext,
    ) -> bool {
        match change.instruction {
            BuilderInstruction::Create(d, _) => {
                context.map.empty(change.target_pos.neighbour(d))
            }
            _ => match &self.next {
                Some(n) => n.handle(change, context),
                None => true,
            },
        }
    }
}