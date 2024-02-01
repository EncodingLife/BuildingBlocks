use bevy::prelude::*;

use crate::mods::{map::settings::MapSettings, organism::Organism};

use super::stem::StemBundle;

pub(super) fn spawn_stem_cell(
    map_settings: Res<MapSettings>,
    mut commands: Commands,
    query: Query<(Entity, &Organism), Added<Organism>>,
) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;


    if !query.is_empty() {
        println!("Organism Entities Spawning")
    }
    for (e, o) in query.iter() {
        let child = commands
            .spawn(StemBundle::new(
                o.starting_position,
                map_settings.cell_width,
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
