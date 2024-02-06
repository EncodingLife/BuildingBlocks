use bevy::ecs::system::Resource;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const MAP_CELL_WIDTH: u16 = 20;
pub const MAP_CELL_HEIGHT: u16 = 20;
pub const TICK_RATE: u64 = 4;

pub const TEMP_STARTING_ENERGY: u16 = 3;
pub const ORGANELLE_STEM_TICK_RATE: u16 = 3;
pub const ORGANELLE_NUCLEUS_TICK_RATE: u16 = 10;
pub const ORGANELLE_MEMBRANE_TICK_RATE: u16 = 6;
pub const ORGANELLE_CHLOROPLAST_TICK_RATE: u16 = 5;



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