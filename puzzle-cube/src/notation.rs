use anyhow::anyhow;
use itertools::Itertools;

use super::cube::{face::Face, rotation::Rotation, Cube};

const CHAR_FOR_ANTICLOCKWISE: char = '\'';
const CHAR_FOR_TURN_TWICE: char = '2';

/// Perform a sequence of moves on a provided Cube instance.
/// # Errors
/// Will return an Err variant when the input `token_sequence` is malformed
pub fn perform_3x3_sequence(token_sequence: &str, cube: &mut Cube) -> anyhow::Result<()> {
    token_sequence
        .trim()
        .split(' ')
        .map(parse_token)
        .flatten_ok()
        .try_for_each(|rotation_result| cube.rotate(rotation_result?))
}

fn parse_token(token: &str) -> anyhow::Result<Vec<Rotation>> {
    // todo support more than just 3x3x3 moves (hence Vec<_> signature)
    //  eg support 4x4x4 notation, such as cube_in_cube_etc: B' M2 U2 M2 B F2 R U' R U R2 U R2 F' U F' Uw Lw Uw' Fw2 Dw Rw' Uw Fw Dw2 Rw2
    //  and bigger cube notation, such as 3Bw2 3Rw' 3Fw

    let base_token = get_base_token_if_valid(token);

    let face = match base_token {
        Some('F') => Face::Front,
        Some('R') => Face::Right,
        Some('U') => Face::Up,
        Some('L') => Face::Left,
        Some('B') => Face::Back,
        Some('D') => Face::Down,
        _ => return Err(anyhow!("Unsupported token in notation string: [{token}]")),
    };

    let rotation = if token.ends_with(CHAR_FOR_ANTICLOCKWISE) {
        Rotation::anticlockwise(face)
    } else {
        Rotation::clockwise(face)
    };

    if token.ends_with(CHAR_FOR_TURN_TWICE) {
        Ok(vec![rotation; 2])
    } else {
        Ok(vec![rotation])
    }
}

fn get_base_token_if_valid(token: &str) -> Option<char> {
    let is_valid_2_char_token = token.len() == 2
        && (token.ends_with(CHAR_FOR_ANTICLOCKWISE) || token.ends_with(CHAR_FOR_TURN_TWICE));

    if token.len() == 1 || is_valid_2_char_token {
        token.chars().next()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::cubie_face::CubieFace;
    use crate::{create_cube_from_sides, create_cube_side};

    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_invalid_token {
        ($($name:ident: $value:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let expected_error_msg = format!("Unsupported token in notation string: [{}]", $value);
                    let error = parse_token($value).unwrap_err();
                    assert_eq!(expected_error_msg, format!("{}", error));
                }
            )*
        }
    }

    macro_rules! test_invalid_sequence {
        ($($name:ident: $value:expr, $err_token:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut cube = Cube::create(3.try_into().expect("known good value"));
                    let expected_error_msg = format!("Unsupported token in notation string: [{}]", $err_token);
                    let error = perform_3x3_sequence($value, &mut cube).unwrap_err();
                    assert_eq!(expected_error_msg, format!("{}", error));
                }
            )*
        }
    }

    test_invalid_token!(
        test_invalid_token_m: "M",
        test_invalid_token_f_0: "F0",
        test_invalid_token_f_1: "F1",
        test_invalid_token_f_1_prime: "F1'",
        test_invalid_token_f_2_prime: "F2'",
        test_invalid_token_f_prime_1: "F'1",
        test_invalid_token_f_prime_2: "F'2",
        test_invalid_token_f_3: "F3",
        test_invalid_token_f_f: "FF",
        test_invalid_token_f_f_1: "FF1",
        test_invalid_token_f_f_2: "FF2",
        test_invalid_token_f_2_2: "F22",
        test_invalid_token_1: "1",
        test_invalid_token_2: "2",
        test_invalid_token_3: "3",
    );

    test_invalid_sequence!(
        test_invalid_sequence_too_many_spaces: "F  R U", "",
        test_invalid_sequence_not_enough_spaces: "FR U", "FR",
        test_invalid_sequence_multiple_individual_tokens: "F2' R'' UU", "F2'",
        test_invalid_sequence_invalid_single_char_token: "F2 R G U", "G",
        test_invalid_sequence_invalid_multi_char_token: "F2 R@ U", "R@",
    );

    #[test]
    fn test_perform_3x3_sequence() -> anyhow::Result<()> {
        let mut cube_under_test = Cube::create(3.try_into().expect("known good value"));
        let mut control_cube = Cube::create(3.try_into().expect("known good value"));

        perform_3x3_sequence("F2 R U' F", &mut cube_under_test)
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
    fn test_perform_3x3_sequence_every_token_once() {
        let sequence = "F R U L B D F2 R2 U2 L2 B2 D2 F' R' U' L' B' D'";
        let mut cube_under_test = Cube::create(3.try_into().expect("known good value"));

        perform_3x3_sequence(sequence, &mut cube_under_test)
            .expect("Sequence in test should be valid");

        let expected_cube = create_cube_from_sides!(
            top: create_cube_side!(
                Green Orange Green;
                White White Yellow;
                Blue Red White;
            ),
            bottom: create_cube_side!(
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
}
