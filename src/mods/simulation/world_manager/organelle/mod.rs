use bevy::prelude::*;

use self::change_requests::*;
use self::create::*;
use self::structure_changed::*;

mod change_request_steps;
pub mod change_requests;
pub mod create;
pub mod structure_changed;

pub(super) struct OrganelleEventsPlugin;

impl Plugin for OrganelleEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<OrganelleCreated>()
            .add_event::<OrganelleTypeChange>()
            .add_event::<OrganelleRemoved>()
            .add_systems(Update, handle_organelle_structural_change_requests)
            .add_systems(
                PostUpdate,
                (
                    create_organelle::create_organelle,
                    use_energy::use_energy,
                    organelle_type_changed,
                    organelle_removed,
                ),
            );
    }
}
