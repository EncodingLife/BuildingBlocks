pub mod builder;
pub mod chloroplast;
pub mod nucleus;
pub mod mitochondria;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OrganelleType {
    None,
    Builder,
    Chloroplast,
    Nucleus,
    Mitochondria
}

impl Default for OrganelleType {
    fn default() -> Self {
        OrganelleType::None
    }
}

impl From<u8> for OrganelleType {
    fn from(v: u8) -> Self {
        match v {
            0b000 => OrganelleType::None,
            0b001 => OrganelleType::Builder,
            0b011 => OrganelleType::Chloroplast,
            0b100 => OrganelleType::Nucleus,
            0b101 => OrganelleType::Mitochondria,
            _ => Default::default(),
        }
    }
}

impl From<OrganelleType> for u8 {
    fn from(value: OrganelleType) -> Self {
        match value {
            OrganelleType::None => 0b000,
            OrganelleType::Builder => 0b001,
            OrganelleType::Chloroplast => 0b011,
            OrganelleType::Nucleus => 0b100,
            OrganelleType::Mitochondria => 0b101,
            _ => Default::default(),
        }
    }
}
