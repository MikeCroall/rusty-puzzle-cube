use crate::cube::{Cube, DefaultSide, flat_side::FlatSide};
use pretty_assertions::assert_eq;

use super::cubie_face::CubieFace;

/// Easily create an entire cube in a custom state, useful for testing. Best used in conjunction with [`create_cube_side`].
///
/// The sides provided must be of the same size.
#[macro_export]
macro_rules! create_cube_from_sides {
    (
        flatten,
        up: $up:expr,
        down: $down:expr,
        front: $front:expr,
        right: $right:expr,
        back: $back:expr,
        left: $left:expr $(,)?
    ) => {
        create_cube_from_sides!(
            up: $up.into_iter().flatten().collect(),
            down: $down.into_iter().flatten().collect(),
            front: $front.into_iter().flatten().collect(),
            right: $right.into_iter().flatten().collect(),
            back: $back.into_iter().flatten().collect(),
            left: $left.into_iter().flatten().collect(),
        )
    };
    (
        up: $up:expr,
        down: $down:expr,
        front: $front:expr,
        right: $right:expr,
        back: $back:expr,
        left: $left:expr $(,)?
    ) => {
        Cube::create_from_sides($up, $down, $front, $right, $back, $left)
    };
}

/// Easily create one side of a cube. Useful for creating custom cube states in tests.
///
/// Each line of the side is defined as the colours [`super::CubieFace`] provides, and ended by a semicolon. These will be created without the optional custom display char.
/// ```no_run
/// # use rusty_puzzle_cube::create_cube_side;
/// let side = create_cube_side!(
///     Green Orange Green;
///     White White Yellow;
///     Blue Red White;
/// );
/// ```
#[macro_export]
macro_rules! create_cube_side {
    ($colour:ident ; $side_length:literal) => {
        vec![vec![$crate::cube::cubie_face::CubieFace::$colour(None) ; $side_length] ; $side_length].into_iter().flatten().collect()
    };
    ( $( $($colour:ident)+ ; )+ ) => {
        vec![ $(
            vec![ $($crate::cube::cubie_face::CubieFace::$colour(None),)* ],
        )* ].into_iter().flatten().collect()
    };
}

/// Internal helper to assert that freely constructed sides are all of the same length.
fn assert_side_lengths_eq(
    up: &DefaultSide,
    down: &DefaultSide,
    front: &DefaultSide,
    right: &DefaultSide,
    back: &DefaultSide,
    left: &DefaultSide,
) {
    assert_eq!(up.side_length(), down.side_length());
    assert_eq!(up.side_length(), front.side_length());
    assert_eq!(up.side_length(), right.side_length());
    assert_eq!(up.side_length(), back.side_length());
    assert_eq!(up.side_length(), left.side_length());
}

impl Cube {
    /// Create a new [`Cube`] instance with pre-made [`DefaultSide`] instances, specifically for easily defining test cases.
    #[must_use]
    pub fn create_from_sides(
        up: Vec<CubieFace>,
        down: Vec<CubieFace>,
        front: Vec<CubieFace>,
        right: Vec<CubieFace>,
        back: Vec<CubieFace>,
        left: Vec<CubieFace>,
    ) -> Self {
        let side_length = (up.len() as f64).sqrt().floor() as usize;

        let up = FlatSide::new(side_length, up);
        let down = FlatSide::new(side_length, down);
        let front = FlatSide::new(side_length, front);
        let right = FlatSide::new(side_length, right);
        let back = FlatSide::new(side_length, back);
        let left = FlatSide::new(side_length, left);

        assert_side_lengths_eq(&up, &down, &front, &right, &back, &left);

        Self {
            side_length,
            up,
            down,
            front,
            right,
            back,
            left,
        }
    }
}
