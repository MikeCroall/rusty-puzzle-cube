use crate::cube::{Cube, DefaultSide};
use pretty_assertions::assert_eq;

/// Easily create an entire cube in a custom state, useful for testing. Best used in conjunction with [`create_cube_side`].
///
/// The sides provided must be of the same size.
#[macro_export]
macro_rules! create_cube_from_sides {
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
/// With `CubieFace` in scope, each line of the side is defined as the colours `CubieFace` provides, and ended by a semicolon. These will be created without the optional custom display char.
/// ```no_run
/// # use rusty_puzzle_cube::create_cube_side;
/// use rusty_puzzle_cube::cube::cubie_face::CubieFace;
/// let side = create_cube_side!(
///     Green Orange Green;
///     White White Yellow;
///     Blue Red White;
/// );
/// ```
#[macro_export]
macro_rules! create_cube_side {
    ($colour:ident ; $side_length:literal) => {
        vec![vec![CubieFace::$colour(None) ; $side_length] ; $side_length]
    };
    ( $( $($colour:ident)+ ; )+ ) => {
        vec![ $(
            vec![ $(CubieFace::$colour(None),)* ],
        )* ]
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
    let side_length = up.len();
    assert_side_lengths_for("up", up, side_length);
    assert_side_lengths_for("down", down, side_length);
    assert_side_lengths_for("front", front, side_length);
    assert_side_lengths_for("right", right, side_length);
    assert_side_lengths_for("back", back, side_length);
    assert_side_lengths_for("left", left, side_length);
}

fn assert_side_lengths_for(name: &'static str, side: &DefaultSide, expected: usize) {
    assert_eq!(
        expected,
        side.len(),
        "{name} had outer length {actual} but was expected to have length {expected}",
        actual = side.len(),
    );
    side.iter().enumerate().for_each(|(index, inner)| {
        assert_eq!(
            expected,
            inner.len(),
            "{name} had inner (index {index}) length {actual} but was expected to have length {expected}",
            actual = inner.len(),
        );
    });
}

impl Cube {
    /// Create a new [`Cube`] instance with pre-made [`DefaultSide`] instances, specifically for easily defining test cases.
    #[must_use]
    pub fn create_from_sides(
        up: DefaultSide,
        down: DefaultSide,
        front: DefaultSide,
        right: DefaultSide,
        back: DefaultSide,
        left: DefaultSide,
    ) -> Self {
        assert_side_lengths_eq(&up, &down, &front, &right, &back, &left);

        Self {
            side_length: up.len(),
            up,
            down,
            front,
            right,
            back,
            left,
        }
    }
}
