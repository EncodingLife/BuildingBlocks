use bevy::prelude::*;
use crate::mods::cell::bna::BNA_LENGTH;
use crate::mods::cell::instruction::instruction::Instruction;
use crate::mods::cell::instruction::events::self_destructed::SelfDestructed;
use crate::mods::cell::instruction::events::created::CellCreated;
use crate::mods::map::map_position::MapPosition;
use crate::mods::cell::instruction::events::self_replaced::SelfReplaced;
use crate::mods::organism::Organism;

use super::Stem;

pub fn execute_instructions(mut query: Query<(Entity, &Parent, &mut Stem, &MapPosition)>, mut c_writer: EventWriter<CellCreated>, mut sr_writer: EventWriter<SelfReplaced>, o_query: Query<&Organism>) {
    // if !query.is_empty() {
    //     println!("Executing instructions for {} stem cells", query.iter().count())
    // }

    for (e, p, mut s, position) in query.iter_mut() {
        let instruction = match o_query.get(p.get()) {
            Ok(o) => o.bna.get_instruction(s.current_instruction),
            Err(e) => panic!("{e}")
        };
        s.current_instruction = (s.current_instruction + 1) % BNA_LENGTH as u8;
        // println!("Instruction {instruction:?} for stem {s:?}");
        match instruction {
            Instruction::Create(d, iref) => c_writer.send(CellCreated(p.get(), position.neighbour(d), iref)),
            Instruction::ReplaceSelf(t) => sr_writer.send(SelfReplaced(e, p.get(), t, *position))
        }
    }
}