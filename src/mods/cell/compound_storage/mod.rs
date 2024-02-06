use bevy::ecs::component::Component;

#[derive(Component)]
pub struct VolatileEnergyStorage(pub u16);