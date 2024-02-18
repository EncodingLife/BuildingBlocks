use super::super::*;

#[derive(Component, Default)]
pub struct Mitochondria;

impl Mitochondria {
    pub const STRUCTURE: OrganelleStructure = OrganelleStructure {
        color: Color::ORANGE,
        starting_energy: 5,
        spawn_energy_cost: 2
    };
}

impl OrganelleFunctions for Mitochondria {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
    fn get_structure() -> OrganelleStructure {
        Self::STRUCTURE
    }
}
