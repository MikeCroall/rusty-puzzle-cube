use anyhow::{Context, anyhow};
use itertools::Itertools;

use super::cube::{PuzzleCube, face::Face, rotation::Rotation};

const CHAR_FOR_ANTICLOCKWISE: char = '\'';
const CHAR_FOR_TURN_TWICE: char = '2';
const CHAR_FOR_MULTI_LAYER: char = 'w';

/// Parse a sequence of moves.
///
/// # Errors
/// Will return an `Err` variant when the input `notation` is malformed.
pub fn parse_sequence(notation: &str) -> anyhow::Result<Vec<Rotation>> {
    notation
        .split_whitespace()
        .map(parse_token)
        .flatten_ok()
        .collect()
}

/// Perform a sequence of moves on a provided `PuzzleCube` instance.
///
/// # Errors
/// Will return an `Err` variant when the input `sequence` references layers of the cube that the given cube does not have e.g. 4Uw on a 3x3x3 cube.
///
/// If the given moves are intended for a larger cube than provided but only references faces directly and not inner layers, or only references inner layers that are still within the size of the cube, these will be applied without error.
pub fn perform_sequence<C: PuzzleCube>(
    sequence: Vec<Rotation>,
    cube: &mut C,
) -> anyhow::Result<()> {
    sequence
        .into_iter()
        .try_for_each(|rotation_result| cube.rotate(rotation_result))
}

/// Parse a sequence of moves and perform them on a provided `PuzzleCube` instance.
///
/// # Errors
/// Will return an `Err` variant when the input `notation` is malformed or references layers of the cube that the given cube does not have e.g. 4Uw on a 3x3x3 cube.
///
/// If the given moves are intended for a larger cube than provided but only references faces directly and not inner layers, or only references inner layers that are still within the size of the cube, these will be applied without error.
pub fn perform_notation<C: PuzzleCube>(notation: &str, cube: &mut C) -> anyhow::Result<()> {
    perform_sequence(parse_sequence(notation)?, cube)
}

fn parse_token(original_token: &str) -> anyhow::Result<Vec<Rotation>> {
    let token = original_token.trim();

    let (token, anticlockwise) = strip_suffix(token, CHAR_FOR_ANTICLOCKWISE);
    let (token, turn_twice) = strip_suffix(token, CHAR_FOR_TURN_TWICE);
    if anticlockwise && turn_twice {
        return Err(anyhow!(
            "Failed parsing token: [{original_token}] as 'turn twice' should not be used as well as 'anticlockwise'"
        ));
    }
    let (token, multi_layer) = strip_suffix(token, CHAR_FOR_MULTI_LAYER);

    let (token, face) = strip_face_suffix(token)
        .with_context(|| format!("Failed parsing token: [{original_token}]"))?;

    let multi_layer_count = parse_multi_layer_count(token)
        .with_context(|| format!("Failed parsing token: [{original_token}]"))?
        .or(if multi_layer { Some(2) } else { None });

    let mut rotations = vec![rotation(face, anticlockwise)];

    if let Some(multi_layer_limit) = multi_layer_count {
        for layer in 1..multi_layer_limit {
            rotations.push(rotation_inner(face, anticlockwise, layer));
        }
    }

    if turn_twice {
        rotations.extend_from_within(..);
    }

    Ok(rotations)
}

fn strip_suffix(string: &str, suffix: char) -> (&str, bool) {
    if let Some(remainder) = string.strip_suffix(suffix) {
        return (remainder, true);
    }
    (string, false)
}

fn strip_face_suffix(string: &str) -> anyhow::Result<(&str, Face)> {
    let face = match string.chars().last() {
        None => return Err(anyhow!("Missing face character")),
        Some('F') => Face::Front,
        Some('R') => Face::Right,
        Some('U') => Face::Up,
        Some('L') => Face::Left,
        Some('B') => Face::Back,
        Some('D') => Face::Down,
        Some(c) => return Err(anyhow!("Invalid face character: [{c}]")),
    };

    Ok((&string[..(string.len() - 1)], face))
}

fn parse_multi_layer_count(string: &str) -> anyhow::Result<Option<usize>> {
    if string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(string.parse::<usize>().with_context(|| {
            format!("Invalid multi-layer count: [{string}]")
        })?))
    }
}

fn rotation(face: Face, anticlockwise: bool) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise(face)
    } else {
        Rotation::clockwise(face)
    }
}

fn rotation_inner(face: Face, anticlockwise: bool, layer: usize) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise_setback_from(face, layer)
    } else {
        Rotation::clockwise_setback_from(face, layer)
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::{Cube, cubie_face::CubieFace};
    use crate::{create_cube_from_sides, create_cube_side};

    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_invalid_token {
        ($($name:ident: $value:expr, $err_text:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let error = parse_token($value).unwrap_err();
                    assert!(format!("{:?}", error).starts_with($err_text));
                }
            )*
        }
    }

    macro_rules! test_invalid_sequence {
        ($($name:ident: $value:expr, $err_text:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut cube = Cube::create(3.try_into().expect("known good value"));
                    let error = perform_notation($value, &mut cube).unwrap_err();
                    assert!(format!("{:?}", error).starts_with($err_text));
                }
            )*
        }
    }

    test_invalid_token!(
        test_invalid_token_m: "M", "\
Failed parsing token: [M]

Caused by:
    Invalid face character: [M]",
        test_invalid_token_f_0: "F0", "\
Failed parsing token: [F0]

Caused by:
    Invalid face character: [0]",
        test_invalid_token_f_1: "F1", "\
Failed parsing token: [F1]

Caused by:
    Invalid face character: [1]",
        test_invalid_token_f_1_prime: "F1'", "\
Failed parsing token: [F1']

Caused by:
    Invalid face character: [1]",
        test_invalid_token_f_2_prime: "F2'", "Failed parsing token: [F2'] as 'turn twice' should not be used as well as 'anticlockwise'",
        test_invalid_token_f_prime_1: "F'1", "\
Failed parsing token: [F'1]

Caused by:
    Invalid face character: [1]",
        test_invalid_token_f_prime_2: "F'2", "\
Failed parsing token: [F'2]

Caused by:
    Invalid face character: [']",
        test_invalid_token_f_3: "F3", "\
Failed parsing token: [F3]

Caused by:
    Invalid face character: [3]",
        test_invalid_token_f_f: "FF", "\
Failed parsing token: [FF]

Caused by:
    0: Invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_token_f_f_1: "FF1", "\
Failed parsing token: [FF1]

Caused by:
    Invalid face character: [1]",
        test_invalid_token_f_f_2: "FF2", "\
Failed parsing token: [FF2]

Caused by:
    0: Invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_token_f_2_2: "F22", "\
Failed parsing token: [F22]

Caused by:
    Invalid face character: [2]",
        test_invalid_token_1: "1", "\
Failed parsing token: [1]

Caused by:
    Invalid face character: [1]",
        test_invalid_token_2: "2", "\
Failed parsing token: [2]

Caused by:
    Missing face character",
        test_invalid_token_3: "3", "\
Failed parsing token: [3]

Caused by:
    Invalid face character: [3]",
    );

    test_invalid_sequence!(
        test_invalid_sequence_not_enough_spaces: "FR U", "\
Failed parsing token: [FR]

Caused by:
    0: Invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_sequence_multiple_individual_tokens: "F2' R'' UU", "Failed parsing token: [F2'] as 'turn twice' should not be used as well as 'anticlockwise'",
        test_invalid_sequence_invalid_single_char_token: "F2 R G U", "\
Failed parsing token: [G]

Caused by:
    Invalid face character: [G]",
        test_invalid_sequence_invalid_multi_char_token: "F2 R@ U", "\
Failed parsing token: [R@]

Caused by:
    Invalid face character: [@]",
    );

    #[test]
    fn test_perform_3x3_notation() -> anyhow::Result<()> {
        let mut cube_under_test = Cube::create(3.try_into().expect("known good value"));
        let mut control_cube = Cube::create(3.try_into().expect("known good value"));

        perform_notation("F2 R U' F", &mut cube_under_test)
            .expect("Sequence in test should be valid");

        control_cube.rotate(Rotation::clockwise(Face::Front))?;
        control_cube.rotate(Rotation::clockwise(Face::Front))?;
        control_cube.rotate(Rotation::clockwise(Face::Right))?;
        control_cube.rotate(Rotation::anticlockwise(Face::Up))?;
        control_cube.rotate(Rotation::clockwise(Face::Front))?;

        assert_eq!(control_cube, cube_under_test);
        Ok(())
    }

    #[test]
    fn test_perform_3x3_notation_every_token_once() {
        let sequence = "F R U L B D F2 R2 U2 L2 B2 D2 F' R' U' L' B' D'";
        let mut cube_under_test = Cube::create(3.try_into().expect("known good value"));

        perform_notation(sequence, &mut cube_under_test).expect("Sequence in test should be valid");

        let expected_cube = create_cube_from_sides!(
            up: create_cube_side!(
                Green Orange Green;
                White White Yellow;
                Blue Red White;
            ),
            down: create_cube_side!(
                Orange Yellow Yellow;
                White Yellow Blue;
                White Red Blue;
            ),
            front: create_cube_side!(
                Orange Yellow Green;
                White Blue Green;
                White Blue Red;
            ),
            right: create_cube_side!(
                Red Green Yellow;
                Red Orange Yellow;
                Blue Orange Red;
            ),
            back: create_cube_side!(
                Red Green Orange;
                Orange Green White;
                White Blue Green;
            ),
            left: create_cube_side!(
                Yellow Orange Yellow;
                Blue Red Green;
                Orange Red Blue;
            ),
        );

        assert_eq!(expected_cube, cube_under_test);
    }

    #[test]
    fn test_perform_notation_cube_too_small() {
        let sequence = "4Uw";
        let mut cube = Cube::create(3.try_into().expect("known good value"));

        let error = perform_notation(sequence, &mut cube).unwrap_err();

        assert!(
            format!("{:?}", error)
                .starts_with("side did not have required layer (3 of outer vec of side)")
        );
    }

    #[test]
    fn parse_token_large_cubes_uw() -> anyhow::Result<()> {
        let rotations = parse_token("Uw")?;

        assert_eq!(
            vec![
                Rotation::clockwise(Face::Up),
                Rotation::clockwise_setback_from(Face::Up, 1),
            ],
            rotations
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_fw() -> anyhow::Result<()> {
        let rotations = parse_token("3Fw")?;

        assert_eq!(
            vec![
                Rotation::clockwise(Face::Front),
                Rotation::clockwise_setback_from(Face::Front, 1),
                Rotation::clockwise_setback_from(Face::Front, 2),
            ],
            rotations
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_rw_prime() -> anyhow::Result<()> {
        let rotations = parse_token("3Rw'")?;

        assert_eq!(
            vec![
                Rotation::anticlockwise(Face::Right),
                Rotation::anticlockwise_setback_from(Face::Right, 1),
                Rotation::anticlockwise_setback_from(Face::Right, 2),
            ],
            rotations
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_bw_2() -> anyhow::Result<()> {
        let rotations = parse_token("3Bw2")?;

        assert_eq!(
            vec![
                Rotation::clockwise(Face::Back),
                Rotation::clockwise_setback_from(Face::Back, 1),
                Rotation::clockwise_setback_from(Face::Back, 2),
                Rotation::clockwise(Face::Back),
                Rotation::clockwise_setback_from(Face::Back, 1),
                Rotation::clockwise_setback_from(Face::Back, 2),
            ],
            rotations
        );

        Ok(())
    }
}
