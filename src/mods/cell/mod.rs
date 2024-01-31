use self::{
    instruction::InstructionPlugin,
    stem::{instruction_execution::execute_instructions, StemBundle},
};
use bevy::prelude::*;
use rand::RngCore;

use super::{
    map::{self, map_position::MapPosition, settings::MapSettings},
    tick::Ticked,
};

mod chloroplast;
mod instruction;
mod membrane;
mod stem;
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

    let n = 64 as i32;

    let x_step = map_settings.width as i32 / n;
    let y_step = map_settings.height as i32 / n;

    let mut rand = rand::thread_rng();
    for i in 15..16 {
        for j in 0..(n - 1) {
            commands.spawn(StemBundle::new(
                MapPosition::new(x_step * (1 + i), y_step * (1 + j)),
                map_settings.cell_width,
                start_x,
                start_y,
                0b001_000_01_001_000_01_001_000_01,
            ));
        }
    }
}
