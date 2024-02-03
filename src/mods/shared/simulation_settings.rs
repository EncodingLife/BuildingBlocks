use bevy::ecs::system::Resource;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const MAP_CELL_WIDTH: u16 = 128;
pub const MAP_CELL_HEIGHT: u16 = 128;
pub const TICK_RATE: u64 = 4;




#[derive(Resource, Copy, Clone)]
pub struct SimulationSettings {
    pub cell_size: f32
}

impl SimulationSettings {
    pub fn new() -> Self {
        println!("cell_size: {}", WINDOW_HEIGHT/Into::<f32>::into(MAP_CELL_HEIGHT));
        Self {
            cell_size:  WINDOW_HEIGHT/Into::<f32>::into(MAP_CELL_HEIGHT)
        }
    }
}