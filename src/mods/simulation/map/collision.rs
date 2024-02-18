use bevy::prelude::*;
use rand::{Rng, RngCore};

use super::{super::settings::*, position::MapPosition};

#[derive(Resource)]
pub struct CollisionMap {
    occupancy: [u8; MAP_CELL_HEIGHT as usize * MAP_CELL_WIDTH as usize],
}

impl CollisionMap {
    // #TODO: Fix the inconsistency in cell co-ordinate data types
    pub fn new() -> Self {
        Self {
            occupancy: [0; (MAP_CELL_HEIGHT as usize * MAP_CELL_WIDTH as usize)],
        }
    }

    pub fn length() -> u32 {
        (MAP_CELL_HEIGHT * MAP_CELL_WIDTH) as u32
    }

    fn to_1_d(x: u16, y: u16) -> usize {
        let k = ((MAP_CELL_WIDTH * y) + x) as usize;
        assert!(
            k < MAP_CELL_HEIGHT as usize * MAP_CELL_WIDTH as usize,
            "{x},{y}",
        );
        k
    }

    fn get_map_position_from_1_d(pos: u16) -> MapPosition {
        let y = pos % MAP_CELL_WIDTH;
        let x = (pos - y) / MAP_CELL_WIDTH;
        MapPosition::new(x as u32, y as u32)
    }

    pub fn get(&self, x: u16, y: u16) -> u8 {
        assert!(x < MAP_CELL_WIDTH && y < MAP_CELL_HEIGHT, "{x},{y}");
        self.occupancy[Self::to_1_d(x, y)]
    }

    pub fn empty(&self, mp: MapPosition) -> bool {
        self.occupancy[Self::to_1_d(mp.x.try_into().unwrap(), mp.y.try_into().unwrap())] == 0
    }

    pub fn set(&mut self, x: u16, y: u16, val: u8) {
        assert!(x < MAP_CELL_WIDTH && y < MAP_CELL_HEIGHT, "{x},{y}");
        self.occupancy[Self::to_1_d(x, y)] = val;
    }

    fn get_neigbours(&self, pos: MapPosition, range: usize) -> Vec<MapPosition> {
        let mut neighbours = vec![];

        for i in 0..(range * 2) {
            let xo = i as i32 - range as i32;
            for j in 0..(range * 2) {
                let yo = j as i32 - range as i32;

                neighbours.push(pos.offset(xo, yo))
            }
        }
        neighbours
    }

    pub fn random_empty_position(&self) -> MapPosition {
        loop {
            let r = MapPosition::new(
                rand::thread_rng().gen_range(0..MAP_CELL_WIDTH).into(),
                rand::thread_rng().gen_range(0..MAP_CELL_HEIGHT).into(),
            );

            if self.get(r.x.try_into().unwrap(), r.y.try_into().unwrap()) == 0 {
                return r;
            }
        }
    }
}
