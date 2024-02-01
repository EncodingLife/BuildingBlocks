use bevy::ecs::system::Resource;

pub const MAP_SIZE_HEIGHT: usize = 256;
pub const MAP_SIZE_WIDTH: usize = 256;
pub const CELL_WIDTH: f32 = 4.0;

#[deprecated(since = "0.0.0", note = "Use static for now for improved performance")]
#[derive(Resource, Copy, Clone)]
pub struct MapSettings {
    pub width: u32,
    pub height: u32,
    pub cell_width: f32,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            width: MAP_SIZE_WIDTH as u32,
            height: MAP_SIZE_HEIGHT as u32,
            cell_width: CELL_WIDTH,
        }
    }
}
