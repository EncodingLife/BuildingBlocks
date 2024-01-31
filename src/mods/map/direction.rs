#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MapDirection {
    // 0b0
    Left,

    // 0b1
    Right,

    // 0b10
    Up,

    // 0b11
    Down,

    // 0b100
    None,
}

impl Default for MapDirection {
    fn default() -> Self {
        MapDirection::Right
    }
}

impl From<u64> for MapDirection {
    fn from(v: u64) -> Self {
        match v {
            0b000 => MapDirection::Left,
            0b001 => MapDirection::Right,
            0b010 => MapDirection::Up,
            0b011 => MapDirection::Down,
            0b100 => MapDirection::None,
            _ => Default::default(),
        }
    }
}