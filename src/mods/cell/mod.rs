use self::{
    instruction::InstructionPlugin,
    maintenance::{consume_energy_for_maintenance, organelle_died::OrganelleDied},
    spawn::spawn_stem_cell,
    stem::{instruction_execution::execute_instructions, StemBundle},
};
use bevy::prelude::*;

use super::tick::Ticked;

pub mod bna;
mod chloroplast;
mod compound_storage;
pub mod instruction;
pub mod maintenance;
mod membrane;
mod nucleus;
mod spawn;
pub mod stem;
mod structure;
pub mod organelle;
pub mod r#type;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InstructionPlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Update, spawn_stem_cell)
            .add_systems(
                Update,
                (execute_instructions, consume_energy_for_maintenance).run_if(on_event::<Ticked>()),
            );
    }
}
