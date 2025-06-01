use crate::{
    cube::{PuzzleCube, rotation::Rotation},
    notation::{parse_sequence, perform_sequence},
};

use strum::EnumIter;

const CHECKERBOARD_CORNERS_3X3X3: &str = "R2 L2 F2 B2 U2 D2";
const CROSSES_3X3X3: &str = "R2 L' D F2 R' D' R' L U' D R D B2 R' U D2";
const NESTED_CUBE_3X3X3: &str = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
const NESTED_CUBE_4X4X4: &str = "B' Lw2 L2 Rw2 R2 U2 Lw2 L2 Rw2 R2 B F2 R U' R U R2 U R2 F' U F' Uw Lw Uw' Fw2 Dw Rw' Uw Fw Dw2 Rw2";

/// A collection of pre-defined sequences that can be applied to `PuzzleCube` instances to achieve visually pleasing patterns.
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum KnownTransform {
    /// Turns a 3x3x3 cube into a checkerboard.
    ///
    /// This can be applied to any cube size, but will pretend the cube is a 3x3x3.
    CheckerboardCorners3x3x3,

    /// Puts a cross (plus sign) on each side of a 3x3x3 cube.
    ///
    /// This can be applied to any cube size, but will pretend the cube is a 3x3x3.
    Crosses3x3x3,

    /// Turns a 3x3x3 cube into 3 nested cubes (cube within a cube within a cube).
    ///
    /// This can be applied to any cube size, but will pretend the cube is a 3x3x3.
    NestedCube3x3x3,

    /// Turns a 4x4x4 cube into 4 nested cubes (cube within a cube within a cube within a cube).
    ///
    /// This can be applied to any cube that is 4x4x4 or larger, but will not have the desired effect on cubes larger than 4x4x4.
    NestedCube4x4x4,
}

impl KnownTransform {
    /// A short name to represent what the transform does.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            KnownTransform::CheckerboardCorners3x3x3 => "Checkerboard Corners",
            KnownTransform::Crosses3x3x3 => "Crosses",
            KnownTransform::NestedCube3x3x3 => "Nested Cubes (3)",
            KnownTransform::NestedCube4x4x4 => "Nested Cubes (4)",
        }
        .to_owned()
    }

    /// A blurb to add extra information for users to better understand what the transform does.
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            KnownTransform::CheckerboardCorners3x3x3 | KnownTransform::Crosses3x3x3 | KnownTransform::NestedCube3x3x3 => {
                "Designed for 3x3x3 cubes, can run on any size cube"
            }
            KnownTransform::NestedCube4x4x4 => {
                "Designed for 4x4x4 cubes, can run on any cube 4x4x4 or larger, but will not have the desired effect on cubes larger than 4x4x4"
            }
        }
        .to_owned()
    }

    /// Some transforms are invalid on too-small cube sizes. In such cases, the minimum valid cube size is given here.
    #[must_use]
    pub fn minimum_side_length(&self) -> Option<usize> {
        match self {
            KnownTransform::NestedCube4x4x4 => Some(4),
            _ => None,
        }
    }

    /// The written notation version of this transform that can be parsed and applied to a `PuzzleCube`.
    #[must_use]
    pub fn notation(&self) -> String {
        match self {
            KnownTransform::CheckerboardCorners3x3x3 => CHECKERBOARD_CORNERS_3X3X3,
            KnownTransform::Crosses3x3x3 => CROSSES_3X3X3,
            KnownTransform::NestedCube3x3x3 => NESTED_CUBE_3X3X3,
            KnownTransform::NestedCube4x4x4 => NESTED_CUBE_4X4X4,
        }
        .to_owned()
    }

    /// The parsed version of this transform's `notation`.
    ///
    /// # Panics
    /// Will panic if the hard-coded notation is invalid.
    #[must_use]
    pub fn sequence(&self) -> Vec<Rotation> {
        parse_sequence(&self.notation()).expect("Known transforms must use valid sequences")
    }

    /// Parse this transform's `notation` and immediately perform the resulting sequence on `cube`.
    ///
    /// # Panics
    /// Will panic if the hard-coded notation is invalid, or performed on a `cube` with a too-short side length. Use `minimum_side_length` to ensure the `cube` has a large enough `side_length` first.
    pub fn perform_instantly<C: PuzzleCube>(&self, cube: &mut C) {
        perform_sequence(self.sequence(), cube).expect("Known transforms must use valid sequences");
    }

    /// Parse this transform's `notation` and perform the resulting sequence on `cube` as a sequence controlled by the `cube` itself.
    ///
    /// # Panics
    /// Will panic if the hard-coded notation is invalid, or performed on a `cube` with a too-short side length. Use `minimum_side_length` to ensure the `cube` has a large enough `side_length` first.
    pub fn perform_seq<C: PuzzleCube>(&self, cube: &mut C) {
        cube.rotate_seq(self.sequence())
            .expect("Known transforms must use valid sequences");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        create_cube_from_sides, create_cube_side,
        cube::{Cube, cubie_face::CubieFace},
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_checkerboard_corners_3x3x3() {
        assert_eq!(
            None,
            KnownTransform::CheckerboardCorners3x3x3.minimum_side_length()
        );

        let mut cube = Cube::create(3.try_into().expect("known good value"));

        KnownTransform::CheckerboardCorners3x3x3.perform_instantly(&mut cube);

        let expected_cube = create_cube_from_sides!(
            up: create_cube_side!(
                White Yellow White;
                Yellow White Yellow;
                White Yellow White;
            ),
            down: create_cube_side!(
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
    fn test_nested_cube_3x3x3() {
        assert_eq!(None, KnownTransform::NestedCube3x3x3.minimum_side_length());

        let mut cube = Cube::create(3.try_into().expect("known good value"));

        KnownTransform::NestedCube3x3x3.perform_instantly(&mut cube);

        let expected_cube = create_cube_from_sides!(
            up: create_cube_side!(
                Blue Blue Blue;
                Blue White White;
                Blue White Orange;
            ),
            down: create_cube_side!(
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
    fn test_nested_cube_4x4x4() {
        assert_eq!(
            Some(4),
            KnownTransform::NestedCube4x4x4.minimum_side_length()
        );

        let mut cube = Cube::create(4.try_into().expect("known good value"));

        KnownTransform::NestedCube4x4x4.perform_instantly(&mut cube);

        let expected_cube = create_cube_from_sides!(
            up: create_cube_side!(
                White White White White;
                White Yellow Yellow Yellow;
                White Yellow Blue Blue;
                White Yellow Blue Green;
            ),
            down: create_cube_side!(
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
