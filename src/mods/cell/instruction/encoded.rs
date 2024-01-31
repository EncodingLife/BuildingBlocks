use crate::mods::{cell::r#type::CellType, map::direction::MapDirection};

use super::instruction::Instruction;

fn decode_bna(mut bna: u64) -> Vec<Instruction> {
    let mut vec = vec![];
    loop {
        let (instruction, new_bna) = decode_instruction(bna);
        bna = new_bna;
        println!("{instruction:?}");

        vec.push(instruction);
        match instruction {
            Instruction::SelfDestruct => break,
            Instruction::ReplaceSelf(c) => break,
            _ => (),
        }
    }
    vec
}

fn decode_instruction(bna: u64) -> (Instruction, u64) {
    let action = bna & 0b11;

    match action {
        0b10 => {
            let cell_type = (bna >> 2) & 0b111;
            (
                Instruction::ReplaceSelf(match cell_type {
                    0b00 => CellType::Nucleus,
                    0b01 => CellType::Membrane,
                    0b10 => CellType::Chloroplast,
                    _ => panic!("Unknown CellType"),
                }),
                bna >> 5,
            )
        }
        0b01 => {
            let cell_type = (bna >> 2) & 0b111;
            let map_direction = (bna >> 2 + 3) & 0b111;
            (
                Instruction::Create(
                    match cell_type {
                        0b00 => CellType::Nucleus,
                        0b01 => CellType::Membrane,
                        0b10 => CellType::Chloroplast,
                        _ => panic!("Unknown CellType {cell_type:#b} in bna {bna:#b}"),
                    },
                    match map_direction {
                        0b000 => MapDirection::Left,
                        0b001 => MapDirection::Right,
                        0b010 => MapDirection::Up,
                        0b011 => MapDirection::Down,
                        0b100 => MapDirection::None,
                        _ => panic!("Unknown Direction"),
                    },
                ),
                bna >> 8,
            )
        }
        0b00 => (Instruction::SelfDestruct, bna >> 2),
        _ => panic!("Unknown Instruction Type"),
    }
}

#[cfg(test)]
mod test {
    use crate::mods::{
        cell::{instruction::instruction::Instruction, r#type::CellType},
        map::direction::MapDirection,
    };

    use super::*;

    #[test]
    fn single_instruction_self_destruct() {
        let result = decode_bna(0b0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Instruction::SelfDestruct);
    }

    #[test]
    fn single_self_replace() {
        // {CellType:3bits} - {ActionType:2bits}
        // Membrane(001)-ReplaceSelf(00)
        let result = decode_bna(0b001_10);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Instruction::ReplaceSelf(CellType::Membrane));
    }

    #[test]
    fn single_create() {
        // {Direction:3bits}-{CellType:3bits}-{ActionType:2bits}
        // Up(010)-Nucleus(000)-ReplaceSelf(00)
        let result = decode_bna(0b010_000_01);

        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Instruction::Create(CellType::Nucleus, MapDirection::Up)
        );
        assert_eq!(result[1], Instruction::SelfDestruct);
    }

    #[test]
    fn multiple_create_then_self_destruct() {
        // {Direction:3bits}-{CellType:3bits}-{ActionType:2bits}
        // SelfDestruct{SelfDestruct(00)} < Create {Up(010)-Nucleus(000)-ReplaceSelf(00)}
        let result = decode_bna(0b0_010_000_01);

        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            Instruction::Create(CellType::Nucleus, MapDirection::Up)
        );
        assert_eq!(result[1], Instruction::SelfDestruct);
    }

    #[test]
    fn multiple_create_then_self_replace() {
        // Chloroplast-ReplaceSelf(0b01010) _ Right-Chloroplast-Create(0b00101001) _ Left-Chloroplast-Create(0b00001001)
        let result = decode_bna(0b01010_00101001_00001001);

        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            Instruction::Create(CellType::Chloroplast, MapDirection::Left)
        );
        assert_eq!(result[1],  Instruction::Create(CellType::Chloroplast, MapDirection::Right));
        assert_eq!(result[2],  Instruction::ReplaceSelf(CellType::Chloroplast));
    }
}
