use std::time::{SystemTime, UNIX_EPOCH};

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

const TICK_RATE: u64 = 28;

#[derive(Event)]
pub struct Ticked();

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Ticked>()
            .add_systems(
                Update,
                tick_timer.run_if(on_timer(Duration::from_millis(1000 / TICK_RATE))),
            );
            // .add_systems(Update, tick_printer.run_if(on_event::<Ticked>()));
    }
}

fn tick_timer(mut ev_tick: EventWriter<Ticked>) {
    ev_tick.send(Ticked());
}

fn tick_printer() {
    println!("Tick {:?}", SystemTime::now());
}
