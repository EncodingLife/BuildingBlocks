use crate::mods::cell::instruction::events::created::*;
use crate::mods::cell::instruction::events::self_replaced::*;
use bevy::prelude::*;

pub mod events;
pub mod instruction;
pub mod set;
pub mod encoded;

pub struct InstructionPlugin;
impl Plugin for InstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellCreated>()
            .add_event::<SelfReplaced>()
            .add_systems(Update, (on_created, on_self_replaced));
    }
}
