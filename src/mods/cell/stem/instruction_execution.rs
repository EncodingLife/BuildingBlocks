use bevy::prelude::*;
use crate::mods::cell::instruction::instruction::Instruction;
use crate::mods::cell::instruction::events::self_destructed::SelfDestructed;
use crate::mods::cell::instruction::events::created::CellCreated;
use crate::mods::map::map_position::MapPosition;
use crate::mods::cell::instruction::events::self_replaced::SelfReplaced;

use super::Stem;

pub fn execute_instructions(mut query: Query<(Entity, &mut Stem, &MapPosition)>, mut c_writer: EventWriter<CellCreated>, mut sr_writer: EventWriter<SelfReplaced>, mut sd_writer: EventWriter<SelfDestructed>) {
    for (e, mut n, position) in query.iter_mut() {
        let instruction = n.instruction_set.next();
        match instruction {
            Instruction::Create(t,d, b) => c_writer.send(CellCreated(t, position.neighbour(d), b)),
            Instruction::ReplaceSelf(t, b) => sr_writer.send(SelfReplaced(e, t, *position, b)),
            Instruction::SelfDestruct => sd_writer.send(SelfDestructed(e)),
        }
    }
}