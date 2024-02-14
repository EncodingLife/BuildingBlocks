use crate::mods::cell::organelle::sustenance::Sustenance;
use crate::mods::cell::organelle::utilizable_energy::UtilizableEnergy;
use bevy::prelude::*;

pub fn consume_energy_for_maintenance(
    mut commands: Commands,
    mut query: Query<(Entity, &Parent, &mut Sustenance, &mut UtilizableEnergy)>,
) {
    for (e, parent, mut m, mut ves) in query.iter_mut() {
        if m.remaining_ticks == 0 {
            m.remaining_ticks = m.tick_rate;
            if ves.0 == 0 {
                commands.entity(parent.get()).remove_children(&[e]);
                commands.entity(e).despawn();
                println!("Organelle {e:?} of {parent:?} despawned")
            } else {
                ves.0 = ves.0 - 1;
            }
        } else {
            m.remaining_ticks = m.remaining_ticks - 1;
        }
    }
}
