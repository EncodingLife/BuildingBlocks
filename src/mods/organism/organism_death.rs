use bevy::prelude::*;

use crate::mods::cell::maintenance::{organelle_died::OrganelleDied, Maintenance};

use super::Organism;

pub fn organelle_died(
    mut command: Commands,
    query: Query<Entity, (With<OrganelleDied>, Without<Children>)>,
    t: Query<&Maintenance>,
) {
    if !query.is_empty() {
        println!(
            "Organisms with recently dead organelle : {}",
            query.iter().count()
        );
    }

    for e in query.iter() {
        println!("Despawning organism {e:?} with no children");
        command.entity(e).despawn()
    }
}
