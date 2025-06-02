#[cfg(test)]
mod quickcheck_tests {
    use crate::cube::{Cube, PuzzleCube, direction::Direction, face::Face, rotation::Rotation};

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
            Rotation {
                relative_to: Face::arbitrary(g),
                layer: usize::arbitrary(g) % CUBE_SIZE,
                direction: Direction::arbitrary(g),
                multilayer: bool::arbitrary(g),
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
}
