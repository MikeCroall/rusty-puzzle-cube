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

    #[derive(Copy, Clone)]
    enum RotationKindNoFields {
        FaceOnly,
        Multilayer,
        Setback,
        MultiSetback,
    }

    impl Arbitrary for RotationKindNoFields {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&[
                RotationKindNoFields::FaceOnly,
                RotationKindNoFields::Multilayer,
                RotationKindNoFields::Setback,
                RotationKindNoFields::MultiSetback,
            ])
            .unwrap()
        }
    }

    impl Arbitrary for RotationKind {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let layer = usize::arbitrary(g) % CUBE_SIZE;
            if layer == 0 {
                return RotationKind::FaceOnly;
            }

            match RotationKindNoFields::arbitrary(g) {
                RotationKindNoFields::FaceOnly => RotationKind::FaceOnly,
                RotationKindNoFields::Multilayer => RotationKind::Multilayer { layer },
                RotationKindNoFields::Setback => RotationKind::Setback { layer },
                RotationKindNoFields::MultiSetback => RotationKind::MultiSetback {
                    start_layer: layer,
                    end_layer: usize::arbitrary(g) % CUBE_SIZE,
                },
            }
        }
    }

    impl Arbitrary for Rotation {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Rotation {
                relative_to: Face::arbitrary(g),
                direction: Direction::arbitrary(g),
                kind: RotationKind::arbitrary(g),
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
    fn single_rotation_changes_two_cubes_identically(rotation: Rotation) -> bool {
        let mut cube_a = Cube::create(CUBE_SIZE.try_into().unwrap());
        let mut cube_b = Cube::create(CUBE_SIZE.try_into().unwrap());

        cube_a.rotate(rotation).unwrap();
        cube_b.rotate(rotation).unwrap();

        cube_a == cube_b
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
