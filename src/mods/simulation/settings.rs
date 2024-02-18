use bevy::ecs::system::Resource;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const MAP_CELL_WIDTH: u16 = 70;
pub const MAP_CELL_HEIGHT: u16 = 40;
pub const MAP_CELL_SIZE: f32 = 18.0; // WINDOW_HEIGHT/Into::<f32>::into(MAP_CELL_HEIGHT);
pub const TICK_RATE: u64 = 16;