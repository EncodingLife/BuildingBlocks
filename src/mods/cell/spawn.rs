use bevy::prelude::*;

use crate::mods::{
    organism::Organism,
    shared::simulation_settings::{SimulationSettings, MAP_CELL_HEIGHT, MAP_CELL_WIDTH},
};

use super::stem::StemBundle;

pub(super) fn spawn_stem_cell(
    simulation_settings: Res<SimulationSettings>,
    mut commands: Commands,
    query: Query<(Entity, &Organism), Added<Organism>>,
) {
    let start_x = (MAP_CELL_WIDTH as f32 * simulation_settings.cell_size) / -2.0;
    let start_y = (MAP_CELL_HEIGHT as f32 * simulation_settings.cell_size) / -2.0;

    if !query.is_empty() {
        println!("Organism Entities Spawning")
    }
    for (e, o) in query.iter() {
        let child = commands
            .spawn(StemBundle::new(
                o.starting_position,
                simulation_settings.cell_size,
                start_x,
                start_y,
                None,
            ))
            .id();
        commands.entity(e).push_children(&[child]);
    }
    if !query.is_empty() {
        println!("Organism Entities Spawning Run Complete")
    }
}
