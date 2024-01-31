use bevy::prelude::*;

#[derive(Event)]
pub struct SelfDestructed(pub Entity);

pub fn on_self_destruct(mut commands: Commands, mut event_reader: EventReader<SelfDestructed>) {
    for ev in event_reader.read() {
        commands.entity(ev.0).despawn()
    }
}