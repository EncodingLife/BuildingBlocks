use crate::mods::cell::bna::BNA;
use crate::mods::cell::chloroplast::ChloroplastBundle;
use crate::mods::cell::membrane::MembraneBundle;
use crate::mods::cell::r#type::CellType;
use crate::mods::cell::MapSettings;
use crate::mods::cell::StemBundle;
use crate::mods::map::collision_map::CollisionMap;
use crate::mods::map::map_position::MapPosition;
use crate::mods::map::settings::CELL_WIDTH;
use crate::mods::map::settings::MAP_SIZE_HEIGHT;
use crate::mods::map::settings::MAP_SIZE_WIDTH;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CellCreated(pub Entity, pub MapPosition, pub u8);

pub fn on_created(
    mut collision_map: ResMut<CollisionMap>,
    mut commands: Commands,
    mut event_reader: EventReader<CellCreated>,
) {
    let start_x = (MAP_SIZE_WIDTH as f32 * CELL_WIDTH) / -2.0;
    let start_y = (MAP_SIZE_HEIGHT as f32 * CELL_WIDTH) / -2.0;

    let mut s = 0;
    let mut f = 0;

    for CellCreated(e, p, instruction_index) in event_reader.read() {
        let occupied = collision_map.get(p.x as usize,p.y as usize) != 0;

        if !occupied {
            s+=1;
            let child = commands
                .spawn(StemBundle::new(
                    *p,
                    CELL_WIDTH,
                    start_x,
                    start_y,
                    Some(*instruction_index),
                ))
                .id();
            match commands.get_entity(*e) {
                Some(mut parent) => {
                    parent.add_child(child);
                }
                None => panic!("get parent resulted in None"),
            };
            collision_map.set(p.x as usize,p.y as usize,CellType::Stem.into());
        } else {
            f+=1;
            // println!("Could not create child in occupied position {p:?} source {e:?}")
        }
    }
    println!("EFCreate: created successfully {s}, failed {f}");
}
