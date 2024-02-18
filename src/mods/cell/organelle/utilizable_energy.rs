use bevy::prelude::*;

#[derive(Copy, Clone, Component, Debug, Reflect, PartialEq)]
pub struct UtilizableEnergy(pub u16);
