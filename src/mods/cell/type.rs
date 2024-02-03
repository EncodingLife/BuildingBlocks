



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellType {
    None,
    Stem,
    Membrane,
    Chloroplast,
    Nucleus
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Membrane
    }
}

impl From<u8> for CellType {
    fn from(v: u8) -> Self {
        match v {
            0b000 => CellType::None,
            0b001 => CellType::Stem,
            0b010 => CellType::Membrane,
            0b011 => CellType::Chloroplast,
            0b100 => CellType::Nucleus,
            _ => Default::default(),
        }
    }
}

impl From<CellType> for u8 {
    fn from(value: CellType) -> Self {
        match value {
            CellType::None => 0b000,
            CellType::Stem => 0b001,
            CellType::Membrane => 0b010,
            CellType::Chloroplast => 0b011,
            CellType::Nucleus => 0b100,
            _ => Default::default(),
        }
    }
}