use super::super::*;

#[derive(Component, Default)]
pub struct Mitochondria;

impl OrganelleStructure for Mitochondria {
    const COLOUR: Color = Color::ORANGE_RED;

    const STARTING_UTILIZABLE_ENERGY: u16 = 5;
}

impl OrganelleFunctions for Mitochondria {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
}
