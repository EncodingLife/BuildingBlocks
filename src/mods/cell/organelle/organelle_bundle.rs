use super::sustenance::*;
use super::*;
use crate::mods::cell::organelle::utilizable_energy::UtilizableEnergy;
use crate::mods::simulation::map::position::MapPosition;
use crate::mods::simulation::settings::MAP_CELL_SIZE;

#[derive(Bundle)]
pub struct OrganelleBundle<T>
where
    T: OrganelleFunctions,
    T: Bundle,
{
    organelle_marker: T,
    sustenance: Sustenance,
    utilizable_energy_store: UtilizableEnergy,
    map_position: MapPosition,
    sprite: SpriteBundle,
}

impl<T> OrganelleBundle<T>
where
    T: Bundle,
    T: OrganelleFunctions,
{
    pub fn new(marker: T, map_position: MapPosition) -> Self {
        Self {
            organelle_marker: marker,
            sustenance: T::get_sustenance(),
            utilizable_energy_store: UtilizableEnergy(T::get_structure().starting_energy),
            map_position,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: T::get_structure().color,
                    custom_size: Some(Vec2::new(MAP_CELL_SIZE, MAP_CELL_SIZE)),
                    ..Default::default()
                },
                global_transform: GlobalTransform::from_translation(map_position.get_translation() + Vec3::new(MAP_CELL_SIZE/2.0, -MAP_CELL_SIZE/2.0, 0.0)),
                ..Default::default()
            },
        }
    }
}
