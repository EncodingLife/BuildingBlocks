use crate::mods::shared::simulation_settings::{MAP_CELL_HEIGHT, MAP_CELL_WIDTH};

use super::{
    cell::bna::BNA, map::map_position::MapPosition, tick::Ticked,
};
use bevy::prelude::*;

pub struct OrganismPlugin;

impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, create_test_organism);
    }
}

#[derive(Component)]
pub struct Organism {
    pub bna: BNA,
    pub starting_position: MapPosition,
}

#[derive(Bundle)]
pub struct OrganismBundle {
    organism: Organism,
    global_transform: GlobalTransform,
    invis: InheritedVisibility,
}

fn create_test_organism(
    mut commands: Commands,
    mut tick_ev_writer: EventWriter<Ticked>,
) {
    // commands.spawn(OrganismBundle {
    //     organism: Organism {
    //         bna: BNA::rand(),
    //         starting_position: MapPosition::new((map_settings.width/2) as i32, (map_settings.height/2) as i32),
    //     },
    //     global_transform: GlobalTransform::IDENTITY,
    //     invis: InheritedVisibility::VISIBLE
    // });

    // return;
    let n = 10;

    let x_step = MAP_CELL_WIDTH / n;
    let y_step = MAP_CELL_HEIGHT / n;

    println!("{x_step}:{y_step}");

    let mut c = 0;

    for i in 0..(n - 1) {
        for j in 0..(n - 1) {
            commands.spawn(OrganismBundle {
                organism: Organism {
                    bna: BNA::rand(),
                    starting_position: MapPosition::new(
                        (x_step * (1 + i)).into(),
                        (y_step * (1 + j)).into(),
                    ),
                },
                global_transform: GlobalTransform::IDENTITY,
                invis: InheritedVisibility::VISIBLE,
            });
            c += 1;
        }
    }
    println!("Total orgs: {c}");
    tick_ev_writer.send(Ticked());
}
