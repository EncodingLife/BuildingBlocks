use crate::mods::{
    cell::{
        builder_instructions::BuilderInstruction,
        organelle::{
            organelle_structural_change_request::OrganelleStructuralChangeRequest,
            types::nucleus::Nucleus,
        },
        Cell,
    },
    simulation::map::position::MapPosition,
};
use bevy::prelude::*;

pub fn queue_nucleus_structural_change_requests(
    mut query: Query<(Entity, &Parent, &mut Nucleus, &MapPosition), Added<Nucleus>>,
    parent_cell_query: Query<&Cell>,
    mut ev_writer: EventWriter<OrganelleStructuralChangeRequest>,
) {
    for (e, p, _, position) in query.iter_mut() {
        println!("Executing initial instructions for {e:?}:");
        let bna = match parent_cell_query.get(p.get()) {
            Ok(o) => o.bna,
            Err(e) => panic!("{e}"),
        };
        for i in 0..4 {
            let i = bna.get_instruction(i);
            println!("- {i:?}:");
            match i {
                BuilderInstruction::Create(d, iref) => {
                    ev_writer.send(OrganelleStructuralChangeRequest {
                        instruction: i,
                        source: e,
                        parent: p.get(),
                        target_pos: position.neighbour(d),
                    });
                }
                _ => (),
            }
        }
    }
}
