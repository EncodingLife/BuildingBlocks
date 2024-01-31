use bevy::ecs::system::Resource;

#[derive(Resource, Copy, Clone)]
pub struct MapSettings {
    pub width: u32,
    pub height: u32,
    pub cell_width: f32
}

impl Default for MapSettings {
    fn default() -> Self {
        Self { width: 64, height: 64, cell_width: 16.0 }
    }
}