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