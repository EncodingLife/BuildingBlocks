use super::{
    cell::bna::BNA, map::{map_position::MapPosition, settings::MapSettings}, tick::Ticked
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
    invis: InheritedVisibility
}

fn create_test_organism(map_settings: Res<MapSettings>, mut commands: Commands, mut tick_ev_writer: EventWriter<Ticked>) {
    let start_x = (map_settings.width as f32 * map_settings.cell_width) / -2.0;
    let start_y = (map_settings.height as f32 * map_settings.cell_width) / -2.0;

    // commands.spawn(OrganismBundle {
    //     organism: Organism {
    //         bna: BNA::rand(),
    //         starting_position: MapPosition::new((map_settings.width/2) as i32, (map_settings.height/2) as i32),
    //     },
    //     global_transform: GlobalTransform::IDENTITY,
    //     invis: InheritedVisibility::VISIBLE
    // });

    // return;
    let n = 15;

    let x_step = map_settings.width / n;
    let y_step = map_settings.height / n;

    println!("{x_step}:{y_step}");

    let mut c = 0;

    for i in 0..(n - 1) {
        for j in 0..(n - 1) {
            commands.spawn(OrganismBundle {
                organism: Organism {
                    bna: BNA::rand(),
                    starting_position: MapPosition::new(x_step * (1 + i), y_step * (1 + j)),
                },
                global_transform: GlobalTransform::IDENTITY,
                invis: InheritedVisibility::VISIBLE
            });
            c+=1;
        }
    }
    println!("Total orgs: {c}");
    tick_ev_writer.send(Ticked());

}
