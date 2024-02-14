use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::settings::TICK_RATE;

#[derive(Event)]
pub struct Ticked();

pub(super) struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Ticked>().add_systems(
            Update,
            tick_timer.run_if(on_timer(Duration::from_millis(1000 / TICK_RATE))),
        );
        // .add_systems(Update, tick_printer.run_if(on_event::<Ticked>()));
    }
}

fn tick_timer(mut ev_tick: EventWriter<Ticked>) {
    ev_tick.send(Ticked());
}
