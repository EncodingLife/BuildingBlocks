use crate::mods::cell::BNA_LENGTH;
use bevy::prelude::*;

use super::super::*;

#[derive(Component, Default)]
pub struct Builder {
    pub instruction_index: u8,
}

impl Builder {
    pub const STRUCTURE: OrganelleStructure = OrganelleStructure {
        color: Color::ANTIQUE_WHITE,
        starting_energy: 5,
        spawn_energy_cost: 2
    };

    pub fn increment(&mut self) {
        self.instruction_index = (self.instruction_index + 1) % (BNA_LENGTH as u8);
    }
}

impl OrganelleFunctions for Builder {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
    fn get_structure() -> OrganelleStructure {
        Self::STRUCTURE
    }
}
