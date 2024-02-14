use crate::mods::cell::BNA_LENGTH;
use bevy::prelude::*;

use super::super::*;

#[derive(Component, Default)]
pub struct Builder {
    pub instruction_index: u8,
}

impl Builder {
    pub fn increment(&mut self) {
        self.instruction_index = (self.instruction_index + 1) % (BNA_LENGTH as u8);
    }
}

impl OrganelleStructure for Builder {
    const COLOUR: Color = Color::WHITE;

    const STARTING_UTILIZABLE_ENERGY: u16 = 5;
}

impl OrganelleFunctions for Builder {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
}
