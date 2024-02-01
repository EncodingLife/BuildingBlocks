use bevy::ecs::component::Component;

use super::{
    direction::MapDirection,
    settings::{MAP_SIZE_HEIGHT, MAP_SIZE_WIDTH},
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
                (MAP_SIZE_WIDTH as u32 + self.x - 1) % MAP_SIZE_WIDTH as u32,
                self.y,
            ),
            MapDirection::Right => MapPosition::new(
                (MAP_SIZE_WIDTH as u32 + self.x + 1) % MAP_SIZE_WIDTH as u32,
                self.y,
            ),
            MapDirection::Up => MapPosition::new(
                self.x,
                (MAP_SIZE_HEIGHT as u32 + self.y + 1) % MAP_SIZE_HEIGHT as u32,
            ),
            MapDirection::Down => MapPosition::new(
                self.x,
                (MAP_SIZE_HEIGHT as u32 + self.y - 1) % MAP_SIZE_HEIGHT as u32,
            ),
        }
    }
}
