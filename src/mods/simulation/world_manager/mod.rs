use bevy::prelude::*;

use self::organelle::OrganelleEventsPlugin;

pub mod organelle;

pub(super) struct WorldManagerPlugin;

impl Plugin for WorldManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(OrganelleEventsPlugin);
    }
}
