use super::super::*;

#[derive(Component, Default)]
pub struct Nucleus;

impl OrganelleStructure for Nucleus {
    const COLOUR: Color = Color::PURPLE;

    const STARTING_UTILIZABLE_ENERGY: u16 = 20;
}

impl OrganelleFunctions for Nucleus {
    fn get_sustenance() -> Sustenance {
        Sustenance::new(10)
    }
}
