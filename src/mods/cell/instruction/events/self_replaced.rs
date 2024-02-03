use crate::mods::cell::chloroplast::ChloroplastBundle;
use crate::mods::cell::membrane::MembraneBundle;
use crate::mods::cell::nucleus::NucleusBundle;
use crate::mods::cell::r#type::CellType;
use crate::mods::cell::StemBundle;
use crate::mods::map::collision_map::CollisionMap;
use crate::mods::map::map_position::MapPosition;
use crate::mods::shared::simulation_settings::SimulationSettings;
use crate::mods::shared::simulation_settings::MAP_CELL_HEIGHT;
use crate::mods::shared::simulation_settings::MAP_CELL_WIDTH;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SelfReplaced(pub Entity, pub Entity, pub CellType, pub MapPosition);

pub fn on_self_replaced(
    mut collision_map: ResMut<CollisionMap>,
    mut commands: Commands,
    mut event_reader: EventReader<SelfReplaced>,
    simulation_settings: Res<SimulationSettings>,
) {
    let start_x = (MAP_CELL_WIDTH as f32 * simulation_settings.cell_size) / -2.0;
    let start_y = (MAP_CELL_HEIGHT as f32 * simulation_settings.cell_size) / -2.0;
    for SelfReplaced(e, parent, ct, mp) in event_reader.read() {
        match commands.get_entity(*e) {
            Some(mut ec) => ec.despawn(),
            None => (), //println!("Attempting to despawn entity {:?} which doesn't exist", e),
        }
        let child = match ct {
            CellType::Membrane => Some(
                commands
                    .spawn(MembraneBundle::new(
                        *mp,
                        simulation_settings.cell_size,
                        start_x,
                        start_y,
                    ))
                    .id(),
            ),
            CellType::Stem => {
                Some(
                    commands
                        .spawn(StemBundle::new(
                            *mp,
                            simulation_settings.cell_size,
                            start_x,
                            start_y,
                            None, // TODO: This may need to change
                        ))
                        .id(),
                )
            }
            CellType::Chloroplast => Some(
                commands
                    .spawn(ChloroplastBundle::new(
                        *mp,
                        simulation_settings.cell_size,
                        start_x,
                        start_y,
                    ))
                    .id(),
            ),
            CellType::Nucleus => Some(
                commands
                    .spawn(NucleusBundle::new(
                        *mp,
                        simulation_settings.cell_size,
                        start_x,
                        start_y,
                    ))
                    .id(),
            ),
            CellType::None => None,
            _ => panic!("Couldnt create from ev {e:?},{ct:?},{mp:?}"),
        };

        match child {
            Some(c) => match commands.get_entity(*parent) {
                Some(mut p) => {
                    p.add_child(c);
                }
                None => panic!("No parent for new entity"),
            },
            None => (),
        }

        collision_map.set(mp.x as u16, mp.y as u16, (*ct).into());
    }
}
