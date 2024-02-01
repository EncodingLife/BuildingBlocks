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
}

impl Default for MapDirection {
    fn default() -> Self {
        MapDirection::Right
    }
}

impl From<u8> for MapDirection {
    fn from(v: u8) -> Self {
        match v {
            0b00 => MapDirection::Left,
            0b01 => MapDirection::Right,
            0b10 => MapDirection::Up,
            0b11 => MapDirection::Down,
            _ => Default::default(),
        }
    }
}


impl From<MapDirection> for u8 {
    fn from(v: MapDirection) -> Self {
        match v {
            MapDirection::Left => 0b00,
            MapDirection::Right => 0b01,
            MapDirection::Up => 0b10,
            MapDirection::Down => 0b11,
            _ => Default::default(),
        }
    }
}