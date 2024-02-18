use super::super::*;

#[derive(Component, Default)]
pub struct Nucleus;

impl Nucleus {
    pub const STRUCTURE: OrganelleStructure = OrganelleStructure {
        color: Color::PURPLE,
        starting_energy: 5,
        spawn_energy_cost: 2
    };
}

impl OrganelleFunctions for Nucleus {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
    fn get_structure() -> OrganelleStructure {
        Self::STRUCTURE
    }
}
