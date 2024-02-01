use bevy::prelude::*;
use super::{map_position::MapPosition, settings::{MAP_SIZE_HEIGHT, MAP_SIZE_WIDTH}};

#[derive(Resource)]
pub struct CollisionMap([u8;MAP_SIZE_HEIGHT*MAP_SIZE_WIDTH]);

impl CollisionMap {
    pub fn new() -> Self {
        Self([0;MAP_SIZE_HEIGHT*MAP_SIZE_WIDTH])
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x<MAP_SIZE_WIDTH && y<MAP_SIZE_HEIGHT, "{x},{y}");
        self.0[(MAP_SIZE_WIDTH*x)+y]
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        assert!(x<MAP_SIZE_WIDTH && y<MAP_SIZE_HEIGHT, "{x},{y}");
        self.0[(MAP_SIZE_WIDTH*x)+y] = val;
    }
}