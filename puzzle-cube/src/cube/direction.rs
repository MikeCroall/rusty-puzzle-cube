use std::ops::Not;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn clockwise_inverted() {
        assert_eq!(Direction::Anticlockwise, !Direction::Clockwise);
    }

    #[test]
    fn anticlockwise_inverted() {
        assert_eq!(Direction::Clockwise, !Direction::Anticlockwise);
    }
}
