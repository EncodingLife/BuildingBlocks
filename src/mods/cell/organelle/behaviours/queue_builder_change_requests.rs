use bevy::prelude::*;

use crate::mods::{
    cell::{
        builder_instructions::BuilderInstruction,
        organelle::{
            organelle_structural_change_request::OrganelleStructuralChangeRequest,
            types::builder::Builder, utilizable_energy::UtilizableEnergy,
        },
        Cell,
    },
    simulation::map::position::MapPosition,
};

pub fn queue_builder_structural_change_requests(
    mut query: Query<(Entity, &Parent, &mut Builder, &MapPosition, &UtilizableEnergy)>,
    parent_cell_query: Query<&Cell>,
    mut ev_writer: EventWriter<OrganelleStructuralChangeRequest>,
) {
    for (e, p, mut b, &mp, &utilizable_energy) in query.iter_mut() {
        let instruction = match parent_cell_query.get(p.get()) {
            Ok(o) => o.bna.get_instruction(b.instruction_index),
            Err(e) => panic!("{e}"),
        };

        b.increment();
        let target_pos = match instruction {
            BuilderInstruction::Create(d, _) => mp.neighbour(d),
            BuilderInstruction::ReplaceSelf(_) => mp,
        };
        ev_writer.send(OrganelleStructuralChangeRequest {
            instruction,
            source: e,
            parent: p.get(),
            target_pos,
            source_energy: utilizable_energy
        });
    }
}
