#[cfg(test)]
mod quickcheck_tests {
    use crate::{
        cube::{
            Cube, PuzzleCube,
            direction::Direction,
            face::Face,
            rotation::{Rotation, RotationKind},
        },
        notation::parse_sequence,
    };

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    const CUBE_SIZE: usize = 9;

    impl Arbitrary for Face {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[
                Face::Up,
                Face::Down,
                Face::Front,
                Face::Right,
                Face::Back,
                Face::Left,
            ])
            .unwrap()
        }
    }

    impl Arbitrary for Direction {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[Direction::Clockwise, Direction::Anticlockwise])
                .unwrap()
        }
    }

    impl Arbitrary for Rotation {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let layer = usize::arbitrary(g) % CUBE_SIZE;

            let kind = if layer == 0 {
                RotationKind::FaceOnly
            } else if bool::arbitrary(g) {
                RotationKind::Multilayer { layer }
            } else {
                RotationKind::Setback { layer }
            };

            Rotation {
                relative_to: Face::arbitrary(g),
                direction: Direction::arbitrary(g),
                kind,
            }
        }
    }

    #[quickcheck]
    fn single_rotation_always_changes_cube(rotation: Rotation) -> bool {
        let original_cube = Cube::create(CUBE_SIZE.try_into().unwrap());
        let mut cube = Cube::create(CUBE_SIZE.try_into().unwrap());

        cube.rotate(rotation).unwrap();

        cube != original_cube
    }

    #[quickcheck]
    fn rotations_then_undone_same_as_no_rotations(rotations: Vec<Rotation>) -> bool {
        let original_cube = Cube::create(CUBE_SIZE.try_into().unwrap());
        let mut cube = Cube::create(CUBE_SIZE.try_into().unwrap());

        let inverse_rotations: Vec<_> = rotations.iter().map(|&r| !r).rev().collect();
        cube.rotate_seq(rotations).unwrap();
        cube.rotate_seq(inverse_rotations).unwrap();

        cube == original_cube
    }

    #[quickcheck]
    fn rotations_to_string_then_parsed_are_the_same(rotation: Rotation) -> bool {
        let to_string = rotation.to_string();

        let parsed_back = parse_sequence(&to_string).unwrap();

        parsed_back.len() == 1 && rotation == *parsed_back.first().unwrap()
    }
}
