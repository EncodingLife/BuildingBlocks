use bevy::prelude::*;

use self::organelle::change_requests::*;
use self::organelle::structure_changed::*;

pub mod organelle;

pub(super) struct WorldManagerPlugin;

impl Plugin for WorldManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<OrganelleCreated>()
            .add_event::<OrganelleTypeChange>()
            .add_event::<OrganelleRemoved>()
            .add_systems(Update, handle_organelle_structural_change_requests)
            .add_systems(
                PostUpdate,
                (organelle_created, organelle_type_changed, organelle_removed),
            );
    }
}
