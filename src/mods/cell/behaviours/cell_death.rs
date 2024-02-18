use bevy::prelude::*;

use super::super::Cell;

pub fn cell_death_due_to_no_organelle(
    mut command: Commands,
    query: Query<Entity, (With<Cell>, Without<Children>)>,
) {
    for e in query.iter() {
        command.entity(e).despawn()
    }
}
