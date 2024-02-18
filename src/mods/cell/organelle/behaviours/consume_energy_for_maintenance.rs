use crate::mods::cell::organelle::sustenance::Sustenance;
use crate::mods::cell::organelle::utilizable_energy::UtilizableEnergy;
use crate::mods::simulation::map::position::MapPosition;
use crate::mods::simulation::world_manager::organelle::change_requests::OrganelleRemoved;
use bevy::prelude::*;

pub fn consume_energy_for_maintenance(
    mut ev_writer: EventWriter<OrganelleRemoved>,
    mut query: Query<(
        Entity,
        &Parent,
        &MapPosition,
        &mut Sustenance,
        &mut UtilizableEnergy,
    )>,
) {
    for (e, parent, &mp, mut m, mut ves) in query.iter_mut() {
        if m.remaining_ticks == 0 {
            m.remaining_ticks = m.tick_rate;
            if ves.0 == 0 {
                ev_writer.send(OrganelleRemoved(e, parent.get(), mp));
            } else {
                ves.0 = ves.0 - 1;
            }
        } else {
            m.remaining_ticks = m.remaining_ticks - 1;
        }
    }
}
