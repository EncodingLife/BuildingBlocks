use bevy::prelude::*;

use crate::mods::{
    cell::organelle::{
        organelle_bundle::OrganelleBundle,
        types::{builder::Builder, nucleus::Nucleus, OrganelleType},
    },
    simulation::map::collision::CollisionMap,
};

use super::OrganelleCreated;

pub fn create_organelle(
    mut commands: Commands,
    mut ev_reader: EventReader<OrganelleCreated>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for OrganelleCreated(position, instruction_index, parent, t, source) in ev_reader.read() {
        let organelle = match t {
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
                p.add_child(organelle);
            }
            None => {
                println!("Cell {parent:?} that was spawning a new organelle was destroyed before the organelle was created");
                commands.entity(organelle).despawn()
            }
        };

        collision_map.set(position.x, position.y, (*t).into());
    }
}
