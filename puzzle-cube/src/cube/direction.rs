use std::ops::Not;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    Anticlockwise,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Clockwise => Direction::Anticlockwise,
            Direction::Anticlockwise => Direction::Clockwise,
        }
    }
}
