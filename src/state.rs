pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub enum CCDirection {
    Left,
    Right,
}

pub struct State {
    dp: Direction,
    cc: CCDirection,
}
