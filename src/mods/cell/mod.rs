use self::{
    instruction::InstructionPlugin,
    spawn::spawn_stem_cell,
    stem::{instruction_execution::execute_instructions, StemBundle},
};
use bevy::prelude::*;


use super::{
    tick::Ticked,
};

pub mod bna;
mod chloroplast;
pub mod instruction;
mod membrane;
mod nucleus;
mod spawn;
pub mod stem;
pub mod r#type;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InstructionPlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Update, spawn_stem_cell)
            .add_systems(Update, execute_instructions.run_if(on_event::<Ticked>()));
    }
}
