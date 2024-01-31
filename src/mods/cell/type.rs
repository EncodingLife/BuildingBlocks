use bevy::prelude::default;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellType {
    STEM,
    Membrane,
    Chloroplast
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Membrane
    }
}

impl From<u64> for CellType {
    fn from(v: u64) -> Self {
        match v {
            0b00 => CellType::STEM,
            0b01 => CellType::Membrane,
            0b10 => CellType::Chloroplast,
            _ => Default::default(),
        }
    }
}