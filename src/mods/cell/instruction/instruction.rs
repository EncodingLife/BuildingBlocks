use std::fmt::Debug;

use crate::mods::{cell::r#type::CellType, map::direction::MapDirection};

#[derive(Copy, Clone, Debug)]
pub enum InstructionAction {
    ReplaceSelf(CellType),
    Create(CellType, MapDirection),
    SelfDestruct,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub action: InstructionAction
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            action: InstructionAction::SelfDestruct
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("action", &self.action)
            .finish()
    }
}
