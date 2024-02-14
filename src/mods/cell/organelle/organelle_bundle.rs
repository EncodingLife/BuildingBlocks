use super::sustenance::*;
use super::*;
use crate::mods::cell::organelle::utilizable_energy::UtilizableEnergy;
use crate::mods::simulation::map::position::MapPosition;
use crate::mods::simulation::settings::MAP_CELL_SIZE;

#[derive(Bundle)]
pub struct OrganelleBundle<T>
where
    T: OrganelleStructure,
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
    T: OrganelleStructure,
    T: OrganelleFunctions,
{
    pub fn new(marker: T, map_position: MapPosition) -> Self {
        Self {
            organelle_marker: marker,
            sustenance: T::get_sustenance(),
            utilizable_energy_store: UtilizableEnergy(T::STARTING_UTILIZABLE_ENERGY),
            map_position,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: T::COLOUR,
                    custom_size: Some(Vec2::new(MAP_CELL_SIZE, MAP_CELL_SIZE)),
                    ..Default::default()
                },
                global_transform: GlobalTransform::from_translation(map_position.get_translation()),
                ..Default::default()
            },
        }
    }
}
