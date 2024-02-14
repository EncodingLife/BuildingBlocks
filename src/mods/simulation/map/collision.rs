use bevy::prelude::*;

use super::super::settings::*;

#[derive(Resource)]
pub struct CollisionMap([u8; (MAP_CELL_HEIGHT as usize * MAP_CELL_WIDTH as usize)]);

impl CollisionMap {
    pub fn new() -> Self {
        Self([0; (MAP_CELL_HEIGHT as usize * MAP_CELL_WIDTH as usize)])
    }

    pub fn get(&self, x: u16, y: u16) -> u8 {
        assert!(x < MAP_CELL_WIDTH && y < MAP_CELL_HEIGHT, "{x},{y}");
        self.0[((MAP_CELL_WIDTH * x) + y) as usize]
    }

    pub fn set(&mut self, x: u16, y: u16, val: u8) {
        assert!(x < MAP_CELL_WIDTH && y < MAP_CELL_HEIGHT, "{x},{y}");
        self.0[((MAP_CELL_WIDTH * x) + y) as usize] = val;
    }
}
