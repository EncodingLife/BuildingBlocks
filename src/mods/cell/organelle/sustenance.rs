use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Sustenance {
    pub tick_rate: u16,
    pub remaining_ticks: u16,
}

impl Sustenance {
    pub fn new(tick_rate: u16) -> Self {
        Self {
            tick_rate,
            remaining_ticks: tick_rate,
        }
    }
}
