use bevy::prelude::*;
use rand::Rng;

use crate::mods::simulation::settings::{MAP_CELL_HEIGHT, MAP_CELL_WIDTH};

use super::{cell::{bna::BNA, organelle::types::OrganelleType, CellSpawned}, interface::simulation_info::SimulationDiagnosticInfo, simulation::map::{collision::CollisionMap, position::MapPosition}};

pub fn debug_setup(mut ev_writer: EventWriter<CellSpawned>) {
    ev_writer.send(CellSpawned(BNA::QuickBuild(OrganelleType::Mitochondria), MapPosition::new(5,5)));
}


pub fn population_control(diagnostics: Res<SimulationDiagnosticInfo>, collision_map: Res<CollisionMap>, mut ev_writer: EventWriter<CellSpawned>) {
    if diagnostics.cell_count < 300 && rand::thread_rng().gen_bool(0.99) {
        let p = collision_map.random_empty_position();
        let t = match  rand::thread_rng().gen_bool(0.5) {
            true => OrganelleType::Chloroplast,
            false => OrganelleType::Mitochondria
        };
        assert!(p.x < MAP_CELL_WIDTH.into());
        assert!(p.y < MAP_CELL_HEIGHT.into());
        ev_writer.send(CellSpawned(BNA::QuickBuild(t), p));
    }
}