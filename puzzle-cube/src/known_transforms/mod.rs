use crate::{cube::Cube, notation::perform_3x3_sequence};

/// # Panics
/// Will panic if local variable `sequence` contains a malformed sequence. This would be considered a bug.
pub fn checkerboard_corners(cube: &mut Cube) {
    let sequence = "R2 L2 F2 B2 U2 D2";
    perform_3x3_sequence(sequence, cube).expect("Known transforms must use valid sequences");
}

/// # Panics
/// Will panic if local variable `sequence` contains a malformed sequence. This would be considered a bug.
pub fn cube_in_cube_in_cube(cube: &mut Cube) {
    let sequence = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
    perform_3x3_sequence(sequence, cube).expect("Known transforms must use valid sequences");
}

#[cfg(test)]
mod tests {
    use crate::cube::cubie_face::CubieFace;
    use crate::{create_cube_from_sides, create_cube_side, cube::Cube};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_checkerboard_corners() {
        let mut cube = Cube::create(3);

        checkerboard_corners(&mut cube);

        let expected_cube = create_cube_from_sides!(
            top: create_cube_side!(
                White Yellow White;
                Yellow White Yellow;
                White Yellow White;
            ),
            bottom: create_cube_side!(
                Yellow White Yellow;
                White Yellow White;
                Yellow White Yellow;
            ),
            front: create_cube_side!(
                Blue Green Blue;
                Green Blue Green;
                Blue Green Blue;
            ),
            right: create_cube_side!(
                Orange Red Orange;
                Red Orange Red;
                Orange Red Orange;
            ),
            back: create_cube_side!(
                Green Blue Green;
                Blue Green Blue;
                Green Blue Green;
            ),
            left: create_cube_side!(
                Red Orange Red;
                Orange Red Orange;
                Red Orange Red;
            ),
        );

        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_cube_in_cube_in_cube() {
        let mut cube = Cube::create(3);

        cube_in_cube_in_cube(&mut cube);

        let expected_cube = create_cube_from_sides!(
            top: create_cube_side!(
                Blue Blue Blue;
                Blue White White;
                Blue White Orange;
            ),
            bottom: create_cube_side!(
                Green Green Green;
                Yellow Yellow Green;
                Red Yellow Green;
            ),
            front: create_cube_side!(
                Orange Blue White;
                Orange Blue Blue;
                Orange Orange Orange;
            ),
            right: create_cube_side!(
                Blue Orange White;
                Orange Orange White;
                White White White;
            ),
            back: create_cube_side!(
                Red Red Red;
                Red Green Green;
                Red Green Yellow;
            ),
            left: create_cube_side!(
                Yellow Yellow Yellow;
                Red Red Yellow;
                Green Red Yellow;
            ),
        );

        assert_eq!(expected_cube, cube);
    }
}
