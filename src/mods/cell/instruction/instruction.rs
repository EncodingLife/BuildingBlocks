use std::fmt::Debug;

use crate::mods::{cell::{r#type::CellType}, map::direction::MapDirection};


// Every Instruction is 8 bits
// From LSB to MSB
// 3 - Cell Type Bits (see CellType)
// 2 - Directional Bits (see MapDirection)
// 3 - Reference Bits (see below)
// Reference bits refer to which instruction in the BNA to pass to the child


// Every Instruction i 8 bits
// From LSB to MSB
// 1 Instruction Type bit
// > When 0
//   - 3 Cell Type Bits
//   - 4 redundant bits
// > When 1
//   - 2 Direction Bits
//   - 5 Instruction Reference Bits
// Reference bits refer to which instruction in the BNA to pass to the child


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    // 0
    ReplaceSelf(CellType),
    // 1
    Create(MapDirection, u8),
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::ReplaceSelf(CellType::Nucleus)
    }
}

impl Instruction {
    pub fn encode(&self) -> u8 {
        let mut r = 0;

        match self {
            Self::ReplaceSelf(t) => r = r ^ (Into::<u8>::into(*t) << 1),
            Self::Create(d, iref) => r = r | 0b1 | (Into::<u8>::into(*d) << 1) | (iref << 3),
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_should_encode_to_0() {
        assert_eq!(Instruction::default().encode(), 0b000000000)
    }

    #[test]
    fn replace_self_nucleus() {
        assert_eq!(
            Instruction::ReplaceSelf(CellType::Nucleus).encode(),
            0b0000_100_0
        )
    }

    #[test]
    fn create_right() {
        println!("{:#b}", Instruction::Create(MapDirection::Right, 0b01011).encode());
        assert_eq!(
            Instruction::Create(MapDirection::Right, 0b01011).encode(),
            0b01011_01_1
        )
    }
}
