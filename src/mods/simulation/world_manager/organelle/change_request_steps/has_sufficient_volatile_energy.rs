use crate::mods::cell::{
    bna::builder_instructions::BuilderInstruction,
    organelle::{
        organelle_structural_change_request::OrganelleStructuralChangeRequest,
        types::builder::Builder, OrganelleFunctions, OrganelleStructure,
    },
};

use super::{OrganelleChangeRequestApprovalHandler, OrganelleChangeRequestContext};

pub struct HasSufficientUtilizableEnergy {
    pub next: Option<Box<dyn OrganelleChangeRequestApprovalHandler>>,
}

impl OrganelleChangeRequestApprovalHandler for HasSufficientUtilizableEnergy {
    fn set_next(&mut self, next: Box<dyn OrganelleChangeRequestApprovalHandler>) {
        self.next = Some(next);
    }

    fn handle(
        &self,
        change: &OrganelleStructuralChangeRequest,
        context: &OrganelleChangeRequestContext,
    ) -> bool {
        if match change.instruction {
            BuilderInstruction::Create(_, _) => {
                change.source_energy.0 < (Builder::get_structure().spawn_energy_cost + 1)
            }
            BuilderInstruction::ReplaceSelf(t) => {
                change.source_energy.0 < t.get_structure().spawn_energy_cost + 1
            }
            _ => false,
        } {
            false
        } else {
            match &self.next {
                Some(n) => n.handle(change, context),
                None => true,
            }
        }
    }
}
