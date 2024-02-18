use crate::mods::{
    cell::{
        bna::builder_instructions::BuilderInstruction,
        organelle::{
            organelle_structural_change_request::OrganelleStructuralChangeRequest,
            types::OrganelleType,
        },
    },
    simulation::map::{collision::CollisionMap, position::MapPosition},
};
use bevy::prelude::*;

#[derive(Event)]
pub struct OrganelleCreated(pub MapPosition, pub u8, pub Entity, pub OrganelleType);

#[derive(Event)]
pub struct OrganelleTypeChange(pub Entity, pub OrganelleType, pub u8, pub MapPosition);

#[derive(Event)]
pub struct OrganelleRemoved(pub Entity, pub Entity, pub MapPosition);

pub fn handle_organelle_structural_change_requests(
    map: Res<CollisionMap>,
    mut ev_reader: EventReader<OrganelleStructuralChangeRequest>,
    mut oc_writer: EventWriter<OrganelleCreated>,
    mut otc_writer: EventWriter<OrganelleTypeChange>,
    mut or_writer: EventWriter<OrganelleRemoved>,
) {
    let valid_instructions = ev_reader
        .read()
        .filter(
            |&OrganelleStructuralChangeRequest {
                 instruction: i,
                 source: e,
                 parent: p,
                 target_pos: mp,
             }| match i {
                BuilderInstruction::ReplaceSelf(t) => true,
                BuilderInstruction::Create(d, iref) => {
                    let pos = mp.neighbour(*d);
                    map.get(pos.x.try_into().unwrap(), pos.y.try_into().unwrap()) == 0
                }
            },
        )
        .collect::<Vec<&OrganelleStructuralChangeRequest>>();

    let grouped = grouped_by_map_position(valid_instructions.clone());

    let solved: Vec<OrganelleStructuralChangeRequest> =
        grouped.iter().map(|(p, ins)| *ins[0]).collect(); // TODO: Add actual solving logic

    if valid_instructions.iter().count() != solved.iter().count() {
        println!("conflict resolution trimmed instructions: {}->{}", valid_instructions.iter().count(), solved.iter().count());
    }

    for &OrganelleStructuralChangeRequest {
        instruction: i,
        source: e,
        parent: p,
        target_pos: mp,
    } in solved.iter()
    {
        match i {
            BuilderInstruction::Create(d, iref) => {
                oc_writer.send(OrganelleCreated(mp, iref, p, OrganelleType::Builder));
            }
            BuilderInstruction::ReplaceSelf(t) => match t {
                crate::mods::cell::organelle::types::OrganelleType::None => {
                    or_writer.send(OrganelleRemoved(e, p, mp));
                }
                _ => otc_writer.send(OrganelleTypeChange(e, t, 0, mp)), // 0 can later be replaced with iref
            },
        }
    }
}

fn grouped_by_map_position(
    mut vec: Vec<&OrganelleStructuralChangeRequest>,
) -> Vec<(MapPosition, Vec<&OrganelleStructuralChangeRequest>)> {
    let mut grouped = vec![];

    while let Some(p) = vec.pop() {
        let mut instructions_for_position: Vec<&OrganelleStructuralChangeRequest> = vec
            .iter()
            .filter(|&&i| p.target_pos == i.target_pos)
            .map(|&i| i)
            .collect();

        instructions_for_position.push(p);

        grouped.push((p.target_pos, instructions_for_position));
        vec.retain(|i| p.target_pos != i.target_pos)
    }

    grouped
}
