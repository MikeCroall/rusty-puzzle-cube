use std::fmt::{Display, Write as _};

use anyhow::{Context, anyhow};
use itertools::Itertools;

use super::cube::{
    PuzzleCube,
    direction::Direction,
    face::Face,
    rotation::{Rotation, RotationKind},
};

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
            "failed parsing token: [{original_token}] as 'turn twice' should not be used as well as 'anticlockwise'"
        ));
    }
    let (token, multi_layer) = strip_suffix(token, CHAR_FOR_MULTI_LAYER);

    let (token, face) = strip_face_suffix(token)
        .with_context(|| format!("failed parsing token: [{original_token}]"))?;

    let multilayer_count = parse_multilayer_count(token)
        .with_context(|| format!("failed parsing token: [{original_token}]"))?
        .or(if multi_layer {
            Some(MultilayerCount::Single(1))
        } else {
            None
        });

    let rotation = if let Some(multilayer_count) = multilayer_count {
        match multilayer_count {
            MultilayerCount::Single(chosen_layer) => {
                if multi_layer {
                    rotation_multilayer(face, anticlockwise, chosen_layer)
                } else {
                    rotation_setback(face, anticlockwise, chosen_layer)
                }
            }
            MultilayerCount::Range(chosen_layer_start, chosen_layer_end) => {
                rotation_multisetback(face, anticlockwise, chosen_layer_start, chosen_layer_end)
            }
        }
    } else {
        rotation(face, anticlockwise)
    };

    let mut rotations = vec![rotation];
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
        None => return Err(anyhow!("missing face character")),
        Some('F') => Face::Front,
        Some('R') => Face::Right,
        Some('U') => Face::Up,
        Some('L') => Face::Left,
        Some('B') => Face::Back,
        Some('D') => Face::Down,
        Some(c) => return Err(anyhow!("invalid face character: [{c}]")),
    };

    Ok((&string[..(string.len() - 1)], face))
}

enum MultilayerCount {
    Single(usize),
    Range(usize, usize),
}

fn parse_multilayer_count(string: &str) -> anyhow::Result<Option<MultilayerCount>> {
    if string.is_empty() {
        return Ok(None);
    }

    let mut split = string.split('-');
    if let (Some(left), Some(right)) = (split.next(), split.next()) {
        return Ok(Some(MultilayerCount::Range(
            left.parse::<usize>()
                .with_context(|| format!("invalid multi-layer range: [{string}]"))?
                - 1,
            right
                .parse::<usize>()
                .with_context(|| format!("invalid multi-layer range: [{string}]"))?
                - 1,
        )));
    }

    Ok(Some(MultilayerCount::Single(
        string
            .parse::<usize>()
            .with_context(|| format!("invalid multi-layer count: [{string}]"))?
            - 1,
    )))
}

fn rotation(face: Face, anticlockwise: bool) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise(face)
    } else {
        Rotation::clockwise(face)
    }
}

fn rotation_setback(face: Face, anticlockwise: bool, layer: usize) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise_setback_from(face, layer)
    } else {
        Rotation::clockwise_setback_from(face, layer)
    }
}

fn rotation_multilayer(face: Face, anticlockwise: bool, layer: usize) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise_multilayer_from(face, layer)
    } else {
        Rotation::clockwise_multilayer_from(face, layer)
    }
}

fn rotation_multisetback(
    face: Face,
    anticlockwise: bool,
    chosen_layer_start: usize,
    chosen_layer_end: usize,
) -> Rotation {
    if anticlockwise {
        Rotation::anticlockwise_multisetback_from(face, chosen_layer_start, chosen_layer_end)
    } else {
        Rotation::clockwise_multisetback_from(face, chosen_layer_start, chosen_layer_end)
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        match self.kind {
            RotationKind::Multilayer { layer } if layer > 1 => {
                let _ = write!(out, "{}", layer + 1);
            }
            RotationKind::Setback { layer } if layer > 0 => {
                let _ = write!(out, "{}", layer + 1);
            }
            RotationKind::MultiSetback {
                start_layer,
                end_layer,
            } => {
                let _ = write!(out, "{}-{}", start_layer + 1, end_layer + 1);
            }
            RotationKind::FaceOnly
            | RotationKind::Multilayer { .. }
            | RotationKind::Setback { .. } => {}
        }
        out.push(match self.relative_to {
            Face::Front => 'F',
            Face::Right => 'R',
            Face::Up => 'U',
            Face::Left => 'L',
            Face::Back => 'B',
            Face::Down => 'D',
        });
        match self.kind {
            RotationKind::Multilayer { layer } if layer > 0 => out.push(CHAR_FOR_MULTI_LAYER),
            RotationKind::FaceOnly
            | RotationKind::Multilayer { .. }
            | RotationKind::Setback { .. }
            | RotationKind::MultiSetback { .. } => {}
        }

        match self.direction {
            Direction::Anticlockwise => {
                let _ = write!(out, "{CHAR_FOR_ANTICLOCKWISE}");
            }
            Direction::Clockwise => {}
        }

        write!(f, "{out}")
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
failed parsing token: [M]

Caused by:
    invalid face character: [M]",
        test_invalid_token_f_0: "F0", "\
failed parsing token: [F0]

Caused by:
    invalid face character: [0]",
        test_invalid_token_f_1: "F1", "\
failed parsing token: [F1]

Caused by:
    invalid face character: [1]",
        test_invalid_token_f_1_prime: "F1'", "\
failed parsing token: [F1']

Caused by:
    invalid face character: [1]",
        test_invalid_token_f_2_prime: "F2'", "failed parsing token: [F2'] as 'turn twice' should not be used as well as 'anticlockwise'",
        test_invalid_token_f_prime_1: "F'1", "\
failed parsing token: [F'1]

Caused by:
    invalid face character: [1]",
        test_invalid_token_f_prime_2: "F'2", "\
failed parsing token: [F'2]

Caused by:
    invalid face character: [']",
        test_invalid_token_f_3: "F3", "\
failed parsing token: [F3]

Caused by:
    invalid face character: [3]",
        test_invalid_token_f_f: "FF", "\
failed parsing token: [FF]

Caused by:
    0: invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_token_f_f_1: "FF1", "\
failed parsing token: [FF1]

Caused by:
    invalid face character: [1]",
        test_invalid_token_f_f_2: "FF2", "\
failed parsing token: [FF2]

Caused by:
    0: invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_token_f_2_2: "F22", "\
failed parsing token: [F22]

Caused by:
    invalid face character: [2]",
        test_invalid_token_1: "1", "\
failed parsing token: [1]

Caused by:
    invalid face character: [1]",
        test_invalid_token_2: "2", "\
failed parsing token: [2]

Caused by:
    missing face character",
        test_invalid_token_3: "3", "\
failed parsing token: [3]

Caused by:
    invalid face character: [3]",
        test_invalid_token_2_dash_f: "2-F", "\
failed parsing token: [2-F]

Caused by:
    0: invalid multi-layer range: [2-]",
        test_invalid_token_dash_2_f: "-2F", "\
failed parsing token: [-2F]

Caused by:
    0: invalid multi-layer range: [-2]"
    );

    test_invalid_sequence!(
        test_invalid_sequence_not_enough_spaces: "FR U", "\
failed parsing token: [FR]

Caused by:
    0: invalid multi-layer count: [F]
    1: invalid digit found in string",
        test_invalid_sequence_multiple_individual_tokens: "F2' R'' UU", "failed parsing token: [F2'] as 'turn twice' should not be used as well as 'anticlockwise'",
        test_invalid_sequence_invalid_single_char_token: "F2 R G U", "\
failed parsing token: [G]

Caused by:
    invalid face character: [G]",
        test_invalid_sequence_invalid_multi_char_token: "F2 R@ U", "\
failed parsing token: [R@]

Caused by:
    invalid face character: [@]",
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
            vec![Rotation::clockwise_multilayer_from(Face::Up, 1)],
            rotations
        );

        assert_eq!(
            "Uw",
            Rotation::clockwise_multilayer_from(Face::Up, 1).to_string()
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_fw() -> anyhow::Result<()> {
        let rotations = parse_token("3Fw")?;

        assert_eq!(
            vec![Rotation::clockwise_multilayer_from(Face::Front, 2)],
            rotations
        );

        assert_eq!(
            "3Fw",
            Rotation::clockwise_multilayer_from(Face::Front, 2).to_string()
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_rw_prime() -> anyhow::Result<()> {
        let rotations = parse_token("3Rw'")?;

        assert_eq!(
            vec![Rotation::anticlockwise_multilayer_from(Face::Right, 2)],
            rotations
        );

        assert_eq!(
            "3Rw'",
            Rotation::anticlockwise_multilayer_from(Face::Right, 2).to_string()
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_3_bw_2() -> anyhow::Result<()> {
        let rotations = parse_token("3Bw2")?;

        assert_eq!(
            vec![
                Rotation::clockwise_multilayer_from(Face::Back, 2),
                Rotation::clockwise_multilayer_from(Face::Back, 2),
            ],
            rotations
        );

        assert_eq!(
            "3Bw",
            Rotation::clockwise_multilayer_from(Face::Back, 2).to_string()
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_4_l_prime() -> anyhow::Result<()> {
        let rotations = parse_token("4L'")?;

        assert_eq!(
            vec![Rotation::anticlockwise_setback_from(Face::Left, 3)],
            rotations
        );

        assert_eq!(
            "4L'",
            Rotation::anticlockwise_setback_from(Face::Left, 3).to_string()
        );

        Ok(())
    }

    #[test]
    fn parse_token_large_cubes_multisetback() -> anyhow::Result<()> {
        let rotations = parse_token("3-6U'")?;

        assert_eq!(
            vec![Rotation::anticlockwise_multisetback_from(Face::Up, 2, 5)],
            rotations
        );

        assert_eq!(
            "3-6U'",
            Rotation::anticlockwise_multisetback_from(Face::Up, 2, 5).to_string()
        );

        Ok(())
    }
}
