use crate::mods::{
    cell::{
        builder_instructions::BuilderInstruction,
        organelle::{
            organelle_structural_change_request::OrganelleStructuralChangeRequest,
            types::nucleus::{InitializedNucleus, Nucleus},
            utilizable_energy::{self, UtilizableEnergy},
        },
        Cell,
    },
    simulation::map::position::MapPosition,
};
use bevy::prelude::*;

pub fn queue_nucleus_structural_change_requests(
    mut query: Query<
        (
            Entity,
            &Parent,
            &mut Nucleus,
            &MapPosition,
            &UtilizableEnergy,
        ),
        Without<InitializedNucleus>,
    >,
    parent_cell_query: Query<&Cell>,
    mut ev_writer: EventWriter<OrganelleStructuralChangeRequest>,
    mut commands: Commands,
) {
    println!("Nucleus tick");
    for (e, p, _, position, &utilizable_energy) in query.iter_mut() {
        println!("New nucleus {e:?}");
        let bna = match parent_cell_query.get(p.get()) {
            Ok(o) => o.bna,
            Err(e) => panic!("{e}"),
        };
        for i in 0..4 {
            let i = bna.get_instruction(i);
            match i {
                BuilderInstruction::Create(d, iref) => {
                    println!("Sending instruction {i:?}");
                    ev_writer.send(OrganelleStructuralChangeRequest {
                        instruction: i,
                        source: e,
                        parent: p.get(),
                        target_pos: position.neighbour(d),
                        source_energy: utilizable_energy,
                    });
                }
                _ => (),
            }
        }
        match commands.get_entity(e) {
            Some(mut entity) => {
                entity.insert(InitializedNucleus);
            }
            _ => (),
        }
    }
}
