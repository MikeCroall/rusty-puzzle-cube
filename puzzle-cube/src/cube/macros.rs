/// Easily create an entire cube in a custom state, useful for testing. Best used in conjunction with [create_cube_side].
///
/// The sides provided must be of the same size.
#[cfg(test)]
#[macro_export]
macro_rules! create_cube_from_sides {
    (
        top: $top:expr,
        bottom: $bottom:expr,
        front: $front:expr,
        right: $right:expr,
        back: $back:expr,
        left: $left:expr $(,)?
    ) => {
        Cube::create_from_sides($top, $bottom, $front, $right, $back, $left)
    };
}

/// Easily create one side of a cube. Useful for creating custom cube states in tests.
///
/// With CubieFace in scope, each line of the side is defined as the colours `CubieFace` provides, and ended by a semicolon. These will be created without the optional custom display char.
/// ```no_run
/// # use rusty_puzzle_cube::create_cube_side;
/// use rusty_puzzle_cube::cube::cubie_face::CubieFace;
/// let side = create_cube_side!(
///     Green Orange Green;
///     White White Yellow;
///     Blue Red White;
/// );
/// ```
#[cfg(test)]
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
