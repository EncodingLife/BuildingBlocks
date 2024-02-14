use crate::mods::cell::organelle::types::OrganelleType;
use crate::mods::simulation::map::direction::MapDirection;

// Every BuildInstruction is 8 bits
// From LSB to MSB
// 3 - Cell Type Bits (see OrganelleType)
// 2 - Directional Bits (see MapDirection)
// 3 - Reference Bits (see below)
// Reference bits refer to which instruction in the BNA to pass to the child

// Every BuildInstruction i 8 bits
// From LSB to MSB
// 1 BuildInstruction Type bit
// > When 0
//   - 3 Cell Type Bits
//   - 4 redundant bits
// > When 1
//   - 2 Direction Bits
//   - 5 BuildInstruction Reference Bits
// Reference bits refer to which instruction in the BNA to pass to the child

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BuilderInstruction {
    // 0
    ReplaceSelf(OrganelleType),
    // 1
    Create(MapDirection, u8),
}

impl Default for BuilderInstruction {
    fn default() -> Self {
        BuilderInstruction::ReplaceSelf(OrganelleType::Nucleus)
    }
}

impl BuilderInstruction {
    pub fn encode(&self) -> u8 {
        let mut r = 0;

        match self {
            Self::ReplaceSelf(t) => r = r ^ (Into::<u8>::into(*t) << 1),
            Self::Create(d, iref) => r = r | 0b1 | (Into::<u8>::into(*d) << 1) | (iref << 3),
        }

        r
    }

    pub fn is_create(&self) -> bool {
        match self {
            Self::ReplaceSelf(_) => false,
            Self::Create(_, _) => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_should_encode_to_0() {
        assert_eq!(BuilderInstruction::default().encode(), 0b000000000)
    }

    #[test]
    fn replace_self_nucleus() {
        assert_eq!(
            BuilderInstruction::ReplaceSelf(OrganelleType::Nucleus).encode(),
            0b0000_100_0
        )
    }

    #[test]
    fn create_right() {
        println!(
            "{:#b}",
            BuilderInstruction::Create(MapDirection::Right, 0b01011).encode()
        );
        assert_eq!(
            BuilderInstruction::Create(MapDirection::Right, 0b01011).encode(),
            0b01011_01_1
        )
    }
}
