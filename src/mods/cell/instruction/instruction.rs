use std::fmt::Debug;

use crate::mods::{cell::r#type::CellType, map::direction::MapDirection};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    // 00
    SelfDestruct,
    // 01
    Create(CellType, MapDirection, u64),
    // 10
    ReplaceSelf(CellType, u64),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::SelfDestruct
    }
}