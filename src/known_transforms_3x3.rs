use crate::{cube::Cube, notation::perform_3x3_sequence};

pub(crate) fn checkerboard_corners(cube: &mut Cube) {
    let sequence = "R2 L2 F2 B2 U2 D2";
    perform_3x3_sequence(sequence, cube);
}

pub(crate) fn cube_in_cube_in_cube(cube: &mut Cube) {
    let sequence = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
    perform_3x3_sequence(sequence, cube);
}
