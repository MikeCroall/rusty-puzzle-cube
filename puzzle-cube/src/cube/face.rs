use std::ops::Not;

use rand::distr::{Distribution, StandardUniform};
use strum::EnumIter;

use self::{Face as F, IndexAlignment as IA};

/// An enum representing the six sides of the cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum Face {
    /// The Up face starts as white cubies
    Up,
    /// The Down face starts as yellow cubies
    Down,
    /// The Front face starts as blue cubies
    Front,
    /// The Right face starts as orange cubies
    Right,
    /// The Back face starts as green cubies
    Back,
    /// The Left face starts as red cubies
    Left,
}

impl Not for Face {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Face::Up => Face::Down,
            Face::Down => Face::Up,
            Face::Front => Face::Back,
            Face::Right => Face::Left,
            Face::Back => Face::Front,
            Face::Left => Face::Right,
        }
    }
}

impl Distribution<Face> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.random_range(0..6) {
            0 => Face::Up,
            1 => Face::Down,
            2 => Face::Front,
            3 => Face::Right,
            4 => Face::Back,
            _ => Face::Left,
        }
    }
}

impl Face {
    /// Provides an array of the four adjacent `Face`s to `self`, each with an `IndexAlignment` that indicates where in the internal 2D `Vec` the cubies that sit along the edge between `self` and the paired `Face` are stored.
    /// See `IndexAlignment` docs for more detail.
    ///
    /// This method is intended for internal use but made available as a convenience.
    #[must_use]
    pub fn adjacent_faces_clockwise(self) -> [(Face, IndexAlignment); 4] {
        match self {
            F::Up => [
                (F::Front, IA::InnerFirst),
                (F::Left, IA::InnerFirst),
                (F::Back, IA::InnerFirst),
                (F::Right, IA::InnerFirst),
            ],
            F::Down => [
                (F::Front, IA::InnerLast),
                (F::Right, IA::InnerLast),
                (F::Back, IA::InnerLast),
                (F::Left, IA::InnerLast),
            ],
            F::Front => [
                (F::Up, IA::InnerLast),
                (F::Right, IA::OuterStart),
                (F::Down, IA::InnerFirst),
                (F::Left, IA::OuterEnd),
            ],
            F::Right => [
                (F::Up, IA::OuterEnd),
                (F::Back, IA::OuterStart),
                (F::Down, IA::OuterEnd),
                (F::Front, IA::OuterEnd),
            ],
            F::Back => [
                (F::Up, IA::InnerFirst),
                (F::Left, IA::OuterStart),
                (F::Down, IA::InnerLast),
                (F::Right, IA::OuterEnd),
            ],
            F::Left => [
                (F::Up, IA::OuterStart),
                (F::Front, IA::OuterStart),
                (F::Down, IA::OuterStart),
                (F::Back, IA::OuterEnd),
            ],
        }
    }
}

/// This enum describes an edge of the 2d side, where a side is a `Vec<Vec<CubieFace>>`.
///
/// For example, given a 3x3 side with numbers representing `CubieFace` instances:
///```text
/// [
///     [0, 1, 2],
///     [3, 4, 5],
///     [6, 7, 8],
/// ]
///```
/// Variants of this enum would represent the following slices:
/// ```text
/// InnerFirst  = 0, 1, 2
/// InnerLast   = 6, 7, 8
/// OuterStart  = 0, 3, 6
/// OuterEnd    = 2, 5, 8
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IndexAlignment {
    /// Indicates that for a `Vec<Vec<_>>`, the desired indices are the 0th index of each inner `Vec`.
    ///
    /// For example, given the below side, this variant would represent positions `0, 3, 6`.
    ///```text
    /// [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ]
    ///```
    OuterStart,
    /// Indicates that for a `Vec<Vec<_>>`, the desired indices are the last index of each inner `Vec`.
    ///
    /// For example, given the below side, this variant would represent positions `2, 5, 8`.
    ///```text
    /// [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ]
    ///```
    OuterEnd,
    /// Indicates that for a `Vec<Vec<_>>`, the desired indices are each index of the inner `Vec` at the outer `Vec`s 0th index.
    ///
    /// For example, given the below side, this variant would represent positions `0, 1, 2`.
    ///```text
    /// [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ]
    ///```
    InnerFirst,
    /// Indicates that for a `Vec<Vec<_>>`, the desired indices are each index of the inner `Vec` at the outer `Vec`s last index.
    ///
    /// For example, given the below side, this variant would represent positions `6, 7, 8`.
    ///```text
    /// [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ]
    ///```
    InnerLast,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn up_inverted() {
        assert_eq!(Face::Down, !Face::Up);
    }

    #[test]
    fn down_inverted() {
        assert_eq!(Face::Up, !Face::Down);
    }

    #[test]
    fn front_inverted() {
        assert_eq!(Face::Back, !Face::Front);
    }

    #[test]
    fn back_inverted() {
        assert_eq!(Face::Front, !Face::Back);
    }

    #[test]
    fn left_inverted() {
        assert_eq!(Face::Right, !Face::Left);
    }

    #[test]
    fn right_inverted() {
        assert_eq!(Face::Left, !Face::Right);
    }
}
