use super::{cube::Cube, notation::perform_sequence};

/// Apply a sequence to the provided cube that will turn a 3x3x3 cube into a checkerboard.
///
/// Can be used on cubes larger than 3x3x3, but only the faces themselves will be rotated. Inner rows/columns will not be rotated.
/// # Panics
/// Will panic if local variable `sequence` contains a malformed sequence. This would be considered a bug.
pub fn checkerboard_corners(cube: &mut Cube) {
    let sequence = "R2 L2 F2 B2 U2 D2";
    perform_sequence(sequence, cube).expect("Known transforms must use valid sequences");
}

/// Apply a sequence to the provided cube that will turn a 3x3x3 cube into a cube within a cube within a cube pattern.
///
/// Can be used on cubes larger than 3x3x3, but only the faces themselves will be rotated. Inner rows/columns will not be rotated.
/// # Panics
/// Will panic if local variable `sequence` contains a malformed sequence. This would be considered a bug.
pub fn cube_in_cube_in_cube(cube: &mut Cube) {
    let sequence = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
    perform_sequence(sequence, cube).expect("Known transforms must use valid sequences");
}

/// Apply a sequence to the provided cube that will turn a 4x4x4 cube into a cube within a cube within a cube pattern.
///
/// Will not produce the desired result on cubes larger than 4x4x4. Stick to the 3x3x3 version for larger cubes, as that is compatible.
/// # Panics
/// Will panic if local variable `sequence` contains a malformed sequence. This would be considered a bug.
pub fn cube_in_cube_in_cube_in_cube(cube: &mut Cube) {
    let sequence = "B' Lw2 L2 Rw2 R2 U2 Lw2 L2 Rw2 R2 B F2 R U' R U R2 U R2 F' U F' Uw Lw Uw' Fw2 Dw Rw' Uw Fw Dw2 Rw2";
    perform_sequence(sequence, cube).expect("Known transforms must use valid sequences");
}

#[cfg(test)]
mod tests {
    use crate::cube::cubie_face::CubieFace;
    use crate::{create_cube_from_sides, create_cube_side, cube::Cube};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_checkerboard_corners() {
        let mut cube = Cube::create(3.try_into().expect("known good value"));

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
        let mut cube = Cube::create(3.try_into().expect("known good value"));

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

    #[test]
    fn test_cube_in_cube_in_cube_in_cube() {
        let mut cube = Cube::create(4.try_into().expect("known good value"));

        cube_in_cube_in_cube_in_cube(&mut cube);

        let expected_cube = create_cube_from_sides!(
            top: create_cube_side!(
                White White White White;
                White Yellow Yellow Yellow;
                White Yellow Blue Blue;
                White Yellow Blue Green;
            ),
            bottom: create_cube_side!(
                Yellow Yellow Yellow Yellow;
                White White White Yellow;
                Green Green White Yellow;
                Blue Green White Yellow;
            ),
            front: create_cube_side!(
                Blue Red Orange Yellow;
                Blue Red Orange Orange;
                Blue Red Red Red;
                Blue Blue Blue Blue;
            ),
            right: create_cube_side!(
                Red White Green Orange;
                White White Green Orange;
                Green Green Green Orange;
                Orange Orange Orange Orange;
            ),
            back: create_cube_side!(
                Green Green Green Green;
                Green Orange Orange Orange;
                Green Orange Red Red;
                Green Orange Red White;
            ),
            left: create_cube_side!(
                Red Red Red Red;
                Blue Blue Blue Red;
                Yellow Yellow Blue Red;
                Orange Yellow Blue Red;
            ),
        );

        assert_eq!(expected_cube, cube);
    }
}
