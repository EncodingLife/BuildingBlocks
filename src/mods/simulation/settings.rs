use bevy::ecs::system::Resource;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const MAP_CELL_WIDTH: u32 = 400;
pub const MAP_CELL_HEIGHT: u32 = 400;
pub const MAP_CELL_SIZE: f32 = 4.0; // WINDOW_HEIGHT/Into::<f32>::into(MAP_CELL_HEIGHT);
pub const TICK_RATE: u64 = 1;