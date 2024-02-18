use super::super::*;

#[derive(Component, Default)]
pub struct Chloroplast;

impl OrganelleStructure for Chloroplast {
    const COLOUR: Color = Color::LIME_GREEN;

    const STARTING_UTILIZABLE_ENERGY: u16 = 18;
}

impl OrganelleFunctions for Chloroplast {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
}
