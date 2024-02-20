use self::{chloroplast::Chloroplast, mitochondria::Mitochondria, nucleus::Nucleus};

use super::{OrganelleFunctions, OrganelleStructure};

pub mod builder;
pub mod chloroplast;
pub mod mitochondria;
pub mod nucleus;

pub use self::builder::Builder;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OrganelleType {
    None,
    Builder,
    Chloroplast,
    Nucleus,
    Mitochondria,
}

impl OrganelleType {
    pub fn get_structure(&self) -> OrganelleStructure {
        match self {
            OrganelleType::None => panic!("Cant get structure of None"),
            OrganelleType::Builder => Builder::get_structure(),
            OrganelleType::Chloroplast => Chloroplast::get_structure(),
            OrganelleType::Nucleus => Nucleus::get_structure(),
            OrganelleType::Mitochondria => Mitochondria::get_structure(),
        }
    }
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
