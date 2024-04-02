use crate::cube::{face::Face as F, Cube};

// TODO refactor to use notation module once stable

pub(crate) fn checkerboard_corners(cube: &mut Cube) {
    cube.rotate_face_90_degrees_clockwise(F::Right);
    cube.rotate_face_90_degrees_clockwise(F::Right);
    cube.rotate_face_90_degrees_anticlockwise(F::Left);
    cube.rotate_face_90_degrees_anticlockwise(F::Left);

    cube.rotate_face_90_degrees_clockwise(F::Front);
    cube.rotate_face_90_degrees_clockwise(F::Front);
    cube.rotate_face_90_degrees_anticlockwise(F::Back);
    cube.rotate_face_90_degrees_anticlockwise(F::Back);

    cube.rotate_face_90_degrees_clockwise(F::Top);
    cube.rotate_face_90_degrees_clockwise(F::Top);
    cube.rotate_face_90_degrees_anticlockwise(F::Bottom);
    cube.rotate_face_90_degrees_anticlockwise(F::Bottom);
}

pub(crate) fn cube_in_cube_in_cube(cube: &mut Cube) {
    cube.rotate_face_90_degrees_clockwise(F::Front);
    cube.rotate_face_90_degrees_anticlockwise(F::Right);
    cube.rotate_face_90_degrees_anticlockwise(F::Top);
    cube.rotate_face_90_degrees_anticlockwise(F::Front);
    cube.rotate_face_90_degrees_clockwise(F::Top);
    cube.rotate_face_90_degrees_anticlockwise(F::Left);
    cube.rotate_face_90_degrees_clockwise(F::Back);
    cube.rotate_face_90_degrees_anticlockwise(F::Top);
    cube.rotate_face_90_degrees_clockwise(F::Back);
    cube.rotate_face_90_degrees_clockwise(F::Back);
    cube.rotate_face_90_degrees_anticlockwise(F::Top);
    cube.rotate_face_90_degrees_anticlockwise(F::Front);
    cube.rotate_face_90_degrees_anticlockwise(F::Right);
    cube.rotate_face_90_degrees_clockwise(F::Back);
    cube.rotate_face_90_degrees_clockwise(F::Right);
    cube.rotate_face_90_degrees_clockwise(F::Right);
    cube.rotate_face_90_degrees_clockwise(F::Front);
    cube.rotate_face_90_degrees_clockwise(F::Top);
    cube.rotate_face_90_degrees_clockwise(F::Left);
    cube.rotate_face_90_degrees_clockwise(F::Top);
}
