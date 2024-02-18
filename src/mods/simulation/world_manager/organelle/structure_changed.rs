use bevy::prelude::*;

use crate::mods::{
    cell::organelle::{
        organelle_bundle::OrganelleBundle,
        types::{
            builder::Builder, chloroplast::Chloroplast, mitochondria::Mitochondria,
            nucleus::Nucleus, OrganelleType,
        },
        utilizable_energy::UtilizableEnergy,
        OrganelleStructure,
    },
    simulation::map::collision::CollisionMap,
};

use super::change_requests::*;

pub fn organelle_created(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleCreated>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for OrganelleCreated(position, instruction_index, parent, t) in ev_reader.read() {
        let o_type = match t {
            OrganelleType::Builder => commands
                .spawn(OrganelleBundle::new(
                    Builder {
                        instruction_index: *instruction_index,
                    },
                    *position,
                ))
                .id(),
            OrganelleType::Nucleus => commands
                .spawn(OrganelleBundle::new(Nucleus, *position))
                .id(),
            _ => panic!("Cant spawn a new {t:?}"),
        };

        match commands.get_entity(*parent) {
            Some(mut p) => {
                p.add_child(o_type);
            }
            None => panic!("get parent resulted in None"),
        }

        collision_map.set(position.x as u16, position.y as u16, (*t).into());
    }
}

pub fn organelle_type_changed(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleTypeChange>,
    mut collision_map: ResMut<CollisionMap>,
    mut organelle_query: Query<(&mut Sprite, &mut UtilizableEnergy)>,
) {
    for &OrganelleTypeChange(entity, o_type, instruction_index, mp) in ev_reader.read() {
        commands
            .entity(entity)
            .remove::<Builder>()
            .remove::<Chloroplast>()
            .remove::<Nucleus>();
        let (new_colour, used_energy) = match o_type {
            OrganelleType::None => {
                panic!("Type cannot be changed to None, should be a removed event")
            }
            OrganelleType::Builder => {
                commands
                    .entity(entity)
                    .insert(Builder { instruction_index });
                (
                    Builder::STRUCTURE.color,
                    Builder::STRUCTURE.spawn_energy_cost,
                )
            }
            OrganelleType::Chloroplast => {
                commands.entity(entity).insert(Chloroplast);
                (
                    Chloroplast::STRUCTURE.color,
                    Chloroplast::STRUCTURE.spawn_energy_cost,
                )
            }
            OrganelleType::Nucleus => {
                commands.entity(entity).insert(Nucleus);
                (
                    Nucleus::STRUCTURE.color,
                    Nucleus::STRUCTURE.spawn_energy_cost,
                )
            }
            OrganelleType::Mitochondria => {
                commands.entity(entity).insert(Mitochondria);
                (
                    Mitochondria::STRUCTURE.color,
                    Mitochondria::STRUCTURE.spawn_energy_cost,
                )
            }
        };
        match organelle_query.get_mut(entity) {
            Ok((mut s, mut ue)) => {
                s.color = new_colour;
                ue.0 = ue.0.checked_sub(used_energy).unwrap();
            }
            Err(_) => panic!("Entity being changes doesnt exist"),
        }

        collision_map.set(mp.x as u16, mp.y as u16, o_type.into());
    }
}

pub fn organelle_removed(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleRemoved>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for &OrganelleRemoved(entity, parent, mp) in ev_reader.read() {
        commands.entity(parent).remove_children(&[entity]);
        commands.entity(entity).despawn();

        collision_map.set(mp.x as u16, mp.y as u16, 0);
    }
}
