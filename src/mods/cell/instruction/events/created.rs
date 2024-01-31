use crate::mods::cell::membrane::MembraneBundle;
use crate::mods::cell::r#type::CellType;
use crate::mods::cell::MapSettings;
use crate::mods::cell::NucleusBundle;
use crate::mods::map::map_position::MapPosition;
use crate::mods::cell::chloroplast::ChloroplastBundle;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct CellCreated(pub CellType, pub MapPosition);

pub fn on_created(
    map_settings: Res<MapSettings>,
    mut commands: Commands,
    mut event_reader: EventReader<CellCreated>,
) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;
    for ev in event_reader.read() {
        match ev.0 {
            CellType::Membrane => commands.spawn(MembraneBundle::new(
                ev.1,
                map_settings.cell_width,
                start_x,
                start_y,
            )),
            CellType::Nucleus => commands.spawn(NucleusBundle::new(
                ev.1,
                map_settings.cell_width,
                start_x,
                start_y,
            )),
            CellType::Chloroplast => commands.spawn(ChloroplastBundle::new(
                ev.1,
                map_settings.cell_width,
                start_x,
                start_y,
            )),
            _ => panic!("Couldnt create from ev {ev:?}"),
        };
    }
}
