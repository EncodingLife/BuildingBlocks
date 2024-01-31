use crate::mods::cell::membrane::MembraneBundle;
use crate::mods::cell::r#type::CellType;
use crate::mods::cell::MapSettings;
use crate::mods::cell::StemBundle;
use crate::mods::map::map_position::MapPosition;
use crate::mods::cell::chloroplast::ChloroplastBundle;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SelfReplaced(pub Entity, pub CellType, pub MapPosition, pub u64);

pub fn on_self_replaced(
    map_settings: Res<MapSettings>,
    mut commands: Commands,
    mut event_reader: EventReader<SelfReplaced>,
) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;
    for ev in event_reader.read() {
        commands.entity(ev.0).despawn();
        match ev.1 {
            CellType::Membrane => commands.spawn(MembraneBundle::new(
                ev.2,
                map_settings.cell_width,
                start_x,
                start_y,
            )),
            CellType::STEM => commands.spawn(StemBundle::new(
                ev.2,
                map_settings.cell_width,
                start_x,
                start_y,
                ev.3
            )),
            CellType::Chloroplast => commands.spawn(ChloroplastBundle::new(
                ev.2,
                map_settings.cell_width,
                start_x,
                start_y,
            )),
            _ => panic!("Couldnt create from ev {ev:?}"),
        };
    }
}
