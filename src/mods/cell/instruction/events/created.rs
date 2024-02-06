use crate::mods::cell::organelle::OrganelleBundle;
use crate::mods::cell::r#type::CellType;
use crate::mods::cell::StemBundle;
use crate::mods::map::collision_map::CollisionMap;
use crate::mods::map::map_position::MapPosition;
use crate::mods::shared::simulation_settings::{
    SimulationSettings, MAP_CELL_HEIGHT, MAP_CELL_WIDTH, ORGANELLE_STEM_TICK_RATE,
    TEMP_STARTING_ENERGY,
};
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CellCreated(pub Entity, pub MapPosition, pub u8);

pub fn on_created(
    mut collision_map: ResMut<CollisionMap>,
    simulation_settings: Res<SimulationSettings>,
    mut commands: Commands,
    mut event_reader: EventReader<CellCreated>,
) {
    let start_x = (MAP_CELL_WIDTH as f32 * simulation_settings.cell_size) / -2.0;
    let start_y = (MAP_CELL_HEIGHT as f32 * simulation_settings.cell_size) / -2.0;

    let mut s = 0;
    let mut f = 0;

    for CellCreated(e, p, instruction_index) in event_reader.read() {
        let occupied = collision_map.get(p.x as u16, p.y as u16) != 0;

        if !occupied {
            s += 1;
            let child = commands
                .spawn(OrganelleBundle::new(
                    StemBundle::new(
                        *p,
                        simulation_settings.cell_size,
                        start_x,
                        start_y,
                        Some(*instruction_index),
                    ),
                    ORGANELLE_STEM_TICK_RATE,
                    TEMP_STARTING_ENERGY,
                ))
                .id();
            match commands.get_entity(*e) {
                Some(mut parent) => {
                    parent.add_child(child);
                }
                None => panic!("get parent resulted in None"),
            };
            collision_map.set(p.x as u16, p.y as u16, CellType::Stem.into());
        } else {
            f += 1;
            // println!("Could not create child in occupied position {p:?} source {e:?}")
        }
    }
    // println!("EFCreate: created successfully {s}, failed {f}");
}
