use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::mods::simulation::tick::Ticked;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PreSim;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FixedBehaviour;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TriggerBehaviour;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ApproveBehaviour;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ExecuteBehaviour;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_schedule(Schedule::new(PreSim));
        app.add_schedule(Schedule::new(FixedBehaviour));
        app.add_schedule(Schedule::new(TriggerBehaviour));
        app.add_schedule(Schedule::new(ApproveBehaviour));
        app.add_schedule(Schedule::new(ExecuteBehaviour));
        app.add_systems(Update, (run_sim).run_if(on_event::<Ticked>()));
    }
}

fn run_sim(world: &mut World) {
    world.run_schedule(PreSim);
    world.run_schedule(FixedBehaviour);
    world.run_schedule(TriggerBehaviour);
    world.run_schedule(ApproveBehaviour);
    world.run_schedule(ExecuteBehaviour);
}
