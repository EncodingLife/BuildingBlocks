use bevy::prelude::*;

use crate::mods::{
    cell::organelle::{
        organelle_bundle::OrganelleBundle,
        types::{builder::Builder, chloroplast::Chloroplast, nucleus::Nucleus, OrganelleType},
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
    for OrganelleCreated(position, instruction_index, parent) in ev_reader.read() {
        let o = commands
            .spawn(OrganelleBundle::new(
                Builder {
                    instruction_index: *instruction_index,
                },
                *position,
            ))
            .id();

        match commands.get_entity(*parent) {
            Some(mut p) => {
                p.add_child(o);
            }
            None => panic!("get parent resulted in None"),
        }

        collision_map.set(
            position.x as u16,
            position.y as u16,
            OrganelleType::Builder.into(),
        );
    }
}

pub fn organelle_type_changed(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleTypeChange>,
    mut collision_map: ResMut<CollisionMap>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for &OrganelleTypeChange(entity, o_type, instruction_index, mp) in ev_reader.read() {
        commands
            .entity(entity)
            .remove::<Builder>()
            .remove::<Chloroplast>()
            .remove::<Nucleus>();
        let c = match o_type {
            OrganelleType::None => {
                panic!("Type cannot be changed to None, should be a removed event")
            }
            OrganelleType::Builder => {
                commands
                    .entity(entity)
                    .insert(Builder { instruction_index });
                Builder::COLOUR
            }
            OrganelleType::Chloroplast => {
                commands.entity(entity).insert(Chloroplast);
                Chloroplast::COLOUR
            }
            OrganelleType::Nucleus => {
                commands.entity(entity).insert(Nucleus);
                Nucleus::COLOUR
            }
        };
        match sprite_query.get_mut(entity) {
            Ok(mut s) => s.color = c,
            Err(_) => panic!("Entity being changes doesnt exist"),
        }

        collision_map.set(mp.x as u16, mp.y as u16, 0);
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