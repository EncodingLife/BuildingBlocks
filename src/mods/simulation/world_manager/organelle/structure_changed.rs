use bevy::prelude::*;

use crate::mods::{
    cell::organelle::{
        types::{
            builder::Builder, chloroplast::Chloroplast, mitochondria::Mitochondria,
            nucleus::Nucleus, OrganelleType,
        },
        utilizable_energy::UtilizableEnergy,
    },
    simulation::map::collision::CollisionMap,
};

use super::change_requests::*;

pub fn organelle_type_changed(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleTypeChange>,
    mut collision_map: ResMut<CollisionMap>,
    mut organelle_query: Query<(&mut Sprite, &mut UtilizableEnergy)>,
) {
    for &OrganelleTypeChange(entity, o_type, _, mp) in ev_reader.read() {
        if let Some(mut e) = commands.get_entity(entity) {
            let o = match o_type {
                OrganelleType::None => {
                    panic!("Type cannot be changed to None, should be a removed event")
                }
                OrganelleType::Builder => None,
                OrganelleType::Chloroplast => {
                    e.insert(Chloroplast);
                    Some((
                        Chloroplast::STRUCTURE.color,
                        Chloroplast::STRUCTURE.spawn_energy_cost,
                    ))
                }
                OrganelleType::Nucleus => {
                    e.insert(Nucleus);
                    Some((
                        Nucleus::STRUCTURE.color,
                        Nucleus::STRUCTURE.spawn_energy_cost,
                    ))
                }
                OrganelleType::Mitochondria => {
                    e.insert(Mitochondria);
                    Some((
                        Mitochondria::STRUCTURE.color,
                        Mitochondria::STRUCTURE.spawn_energy_cost,
                    ))
                }
            };

            if let Some((new_colour, used_energy)) = o {
                e.remove::<Builder>();
                match organelle_query.get_mut(entity) {
                    Ok((mut s, mut ue)) => {
                        s.color = new_colour;
                        ue.0 = ue.0.checked_sub(used_energy).unwrap_or_default();
                    }
                    Err(err) => println!("organelle_type_changed {err}"),
                }

                collision_map.set(mp.x, mp.y, o_type.into());
            }
        } else {
            debug!("Attempting to change type thats already despawned");
        }
    }
}

pub fn organelle_removed(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleRemoved>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for &OrganelleRemoved(entity, parent, mp) in ev_reader.read() {
        collision_map.set(mp.x, mp.y, 0);

        if let Some(mut ec) = commands.get_entity(parent) {
            ec.remove_children(&[entity]);
        };

        if let Some(mut ec) = commands.get_entity(entity) {
            ec.despawn();
        };
    }
}
