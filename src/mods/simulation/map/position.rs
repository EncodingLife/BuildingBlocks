use bevy::prelude::*;

use super::super::settings::*;
use super::direction::*;

const X_START: f32 = (MAP_CELL_WIDTH as f32 * MAP_CELL_SIZE) / -2.0;
const Y_START: f32 = (MAP_CELL_HEIGHT as f32 * MAP_CELL_SIZE) / -2.0;

#[derive(Component, Copy, Clone, Debug, Default, PartialEq, Reflect)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn neighbour(&self, direction: MapDirection) -> Self {
        match direction {
            MapDirection::Left => MapPosition::new(
                (MAP_CELL_WIDTH as u32 + self.x - 1) % MAP_CELL_WIDTH as u32,
                self.y,
            ),
            MapDirection::Right => MapPosition::new(
                (MAP_CELL_WIDTH as u32 + self.x + 1) % MAP_CELL_WIDTH as u32,
                self.y,
            ),
            MapDirection::Up => MapPosition::new(
                self.x,
                (MAP_CELL_HEIGHT as u32 + self.y + 1) % MAP_CELL_HEIGHT as u32,
            ),
            MapDirection::Down => MapPosition::new(
                self.x,
                (MAP_CELL_HEIGHT as u32 + self.y - 1) % MAP_CELL_HEIGHT as u32,
            ),
        }
    }

    pub fn get_translation(&self) -> Vec3 {
        Vec3::new(
            X_START + self.x as f32 * MAP_CELL_SIZE,
            Y_START + self.y as f32 * MAP_CELL_SIZE,
            0.,
        )
    }
}
