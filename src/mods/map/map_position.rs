use bevy::ecs::component::Component;

use super::direction::MapDirection;

#[derive(Component, Copy, Clone, Debug)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
}

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbour(&self, direction: MapDirection) -> Self {
        match direction {
            MapDirection::Left => MapPosition::new(self.x-1, self.y),
            MapDirection::Right => MapPosition::new(self.x+1, self.y),
            MapDirection::Up => MapPosition::new(self.x, self.y+1),
            MapDirection::Down => MapPosition::new(self.x, self.y-1),
            MapDirection::None => MapPosition::new(self.x, self.y),
        }
    }
}