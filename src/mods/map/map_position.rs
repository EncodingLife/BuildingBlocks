use bevy::ecs::component::Component;

use crate::mods::shared::simulation_settings::{MAP_CELL_HEIGHT, MAP_CELL_WIDTH};

use super::{
    direction::MapDirection,
};

#[derive(Component, Copy, Clone, Debug, PartialEq)]
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
}
