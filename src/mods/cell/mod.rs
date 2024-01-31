use bevy::prelude::*;

use self::{instruction::InstructionPlugin, nucleus::{instruction_execution::execute_instructions, NucleusBundle}};

use super::{map::{map_position::MapPosition, settings::MapSettings}, tick::Ticked};

mod instruction;
mod nucleus;
mod membrane;
mod chloroplast;
mod r#type;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InstructionPlugin)
            .add_systems(Startup, create_test_nucleus)
            .add_systems(Update, execute_instructions.run_if(on_event::<Ticked>()));
    }
}

fn create_test_nucleus(map_settings: Res<MapSettings>, mut commands: Commands) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;
    commands.spawn(NucleusBundle::new(
        MapPosition::new(32, 32),
        map_settings.cell_width,
        start_x,
        start_y,
    ));
}
