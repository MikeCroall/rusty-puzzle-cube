use std::collections::HashSet;

use crate::{
    cube::{
        Cube, PuzzleCube,
        cubie_face::CubieFace,
        direction::Direction,
        face::Face,
        rotation::{Rotation, RotationKind},
    },
    notation::parse_sequence,
};

use quickcheck::Arbitrary;
use quickcheck_macros::quickcheck;
use strum::IntoEnumIterator;

/// A const large cube size that allow us to impl [`Arbitrary`] for [`Rotation`] (without any per-test inputs)
/// that is within the limit for using unique characters on each cubie face, for reliable face (in)equality.
///
/// For example, with a new cube that does not use unique characters, turning [`Face::Front`] results in
/// a cube that has an equal [`Face::Front`] to the original. However, using unique characters reveals
/// that indeed the face did rotate, the cubies are just all still the same colour.
const CUBE_SIZE: usize = 8;

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
        let mut layer = usize::arbitrary(g) % CUBE_SIZE;
        if layer == 0 {
            return RotationKind::FaceOnly;
        }

        match RotationKindNoFields::arbitrary(g) {
            RotationKindNoFields::FaceOnly => RotationKind::FaceOnly,
            RotationKindNoFields::Multilayer => RotationKind::Multilayer { layer },
            RotationKindNoFields::Setback => RotationKind::Setback { layer },
            RotationKindNoFields::MultiSetback => {
                let mut end_layer = usize::arbitrary(g) % CUBE_SIZE;
                if layer > end_layer {
                    std::mem::swap(&mut layer, &mut end_layer);
                }
                RotationKind::MultiSetback {
                    start_layer: layer,
                    end_layer,
                }
            }
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

/// Determines the amount of faces of the cube expected to remain unchanged when
/// applying the given [`Rotation`]
fn expected_equal_faces(rotation: Rotation) -> usize {
    match rotation.kind {
        // The entire cube (all slices, including both end faces) gets rotated
        RotationKind::MultiSetback {
            start_layer,
            end_layer,
        } if start_layer == 0 && end_layer == CUBE_SIZE - 1 => 0,
        RotationKind::Multilayer { layer } if layer == CUBE_SIZE - 1 => 0,

        // Some inner slices and one end face get rotated, one end face remains unchanged
        RotationKind::MultiSetback {
            start_layer,
            end_layer,
        } if start_layer == 0 || end_layer == CUBE_SIZE - 1 => 1,
        RotationKind::Setback { layer } if layer == 0 || layer == CUBE_SIZE - 1 => 1,
        RotationKind::FaceOnly | RotationKind::Multilayer { .. } => 1,

        // Only inner slices get rotated, both end faces remain unchanged
        RotationKind::Setback { .. } | RotationKind::MultiSetback { .. } => 2,
    }
}

#[quickcheck]
fn single_rotation_leaves_expected_amount_of_faces_unchanged(rotation: Rotation) -> bool {
    let original_cube = Cube::create_with_unique_characters(CUBE_SIZE.try_into().unwrap());
    let mut cube = Cube::create_with_unique_characters(CUBE_SIZE.try_into().unwrap());

    cube.rotate(rotation).unwrap();

    let equal_faces = Face::iter()
        .filter(|f| cube.side(*f) == original_cube.side(*f))
        .count();

    equal_faces == expected_equal_faces(rotation)
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

#[derive(Default)]
struct UniqueCubieChars {
    blue: HashSet<char>,
    green: HashSet<char>,
    orange: HashSet<char>,
    red: HashSet<char>,
    white: HashSet<char>,
    yellow: HashSet<char>,
}

impl UniqueCubieChars {
    fn all_sides_have_n_unique_chars(self, n: usize) -> bool {
        let UniqueCubieChars {
            blue,
            green,
            orange,
            red,
            white,
            yellow,
        } = self;

        blue.len() == n
            && green.len() == n
            && orange.len() == n
            && red.len() == n
            && white.len() == n
            && yellow.len() == n
    }
}

impl<'a> FromIterator<&'a CubieFace> for UniqueCubieChars {
    fn from_iter<T: IntoIterator<Item = &'a CubieFace>>(iter: T) -> Self {
        let mut cubie_chars = UniqueCubieChars::default();

        for cubie_face in iter {
            match cubie_face {
                CubieFace::Blue(Some(c)) => cubie_chars.blue.insert(*c),
                CubieFace::Green(Some(c)) => cubie_chars.green.insert(*c),
                CubieFace::Orange(Some(c)) => cubie_chars.orange.insert(*c),
                CubieFace::Red(Some(c)) => cubie_chars.red.insert(*c),
                CubieFace::White(Some(c)) => cubie_chars.white.insert(*c),
                CubieFace::Yellow(Some(c)) => cubie_chars.yellow.insert(*c),
                _ => panic!("cannot aggregate cubie chars if cubies do not contain chars"),
            };
        }

        cubie_chars
    }
}

#[quickcheck]
fn cubies_are_never_created_or_destroyed_only_moved(rotations: Vec<Rotation>) -> bool {
    let mut cube = Cube::create_with_unique_characters(CUBE_SIZE.try_into().unwrap());

    cube.rotate_seq(rotations).unwrap();

    let cubie_chars: UniqueCubieChars = Face::iter().flat_map(|f| cube.side(f)).flatten().collect();

    let expected_unique_chars_per_side = CUBE_SIZE * CUBE_SIZE;
    cubie_chars.all_sides_have_n_unique_chars(expected_unique_chars_per_side)
}

#[quickcheck]
fn rotations_to_string_then_parsed_are_the_same(rotation: Rotation) -> bool {
    let to_string = rotation.to_string();

    let parsed_back = parse_sequence(&to_string).unwrap();

    parsed_back.len() == 1 && rotation == *parsed_back.first().unwrap()
}
