#[macro_export]
macro_rules! create_cube_from_sides {
    (
        top: $top:expr,
        bottom: $bottom:expr,
        front: $front:expr,
        right: $right:expr,
        back: $back:expr,
        left: $left:expr,
    ) => {
        Cube::create_from_sides($top, $bottom, $front, $right, $back, $left)
    };
}

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
