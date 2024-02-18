use super::super::*;

#[derive(Component, Default)]
pub struct Chloroplast;

impl Chloroplast {
    pub const STRUCTURE: OrganelleStructure = OrganelleStructure {
        color: Color::GREEN,
        starting_energy: 5,
        spawn_energy_cost: 2
    };
}

impl OrganelleFunctions for Chloroplast {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
    fn get_structure() -> OrganelleStructure {
        Self::STRUCTURE
    }
}
