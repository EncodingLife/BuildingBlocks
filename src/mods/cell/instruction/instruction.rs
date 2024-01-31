use std::fmt::Debug;

use crate::mods::{cell::r#type::CellType, map::direction::MapDirection};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    // 00
    SelfDestruct,
    // 01
    Create(CellType, MapDirection),
    // 10
    ReplaceSelf(CellType),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::SelfDestruct
    }
}