use std::ops::Not;

/// Representing a direction of a rotation about some not-defined-here axis.
///
/// Part of the specification of a rotation on the cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    /// A clockwise rotation relative to some not-defined-here axis.
    Clockwise,
    /// An anti-clockwise rotation relative to some not-defined-here axis.
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
