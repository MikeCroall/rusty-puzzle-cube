use crate::cube::{face::Face, Cube};

const CHAR_FOR_ANTICLOCKWISE: char = '\'';
const CHAR_FOR_TURN_TWICE: char = '2';

// todo support 4x4 notation (needs new cube methods), such as cube_in_cube_etc: B' M2 U2 M2 B F2 R U' R U R2 U R2 F' U F' Uw Lw Uw' Fw2 Dw Rw' Uw Fw Dw2 Rw2

/// # Errors
/// Will return an Err variant when the input `token_sequence` is malformed
pub fn perform_3x3_sequence(token_sequence: &str, cube: &mut Cube) -> Result<(), String> {
    let token_sequence = token_sequence.trim();

    token_sequence
        .trim()
        .split(' ')
        .try_for_each(|token| apply_token(token.trim(), cube))?;

    Ok(())
}

fn apply_token(token: &str, cube: &mut Cube) -> Result<(), String> {
    let base_token = get_base_token_if_valid(token);

    let face = match base_token {
        Some('F') => Ok(Face::Front),
        Some('R') => Ok(Face::Right),
        Some('U') => Ok(Face::Up),
        Some('L') => Ok(Face::Left),
        Some('B') => Ok(Face::Back),
        Some('D') => Ok(Face::Down),
        _ => Err(format!("Unsupported token in notation string: [{token}]")),
    }?;

    let fn_to_apply = if token.ends_with(CHAR_FOR_ANTICLOCKWISE) {
        Cube::rotate_face_90_degrees_anticlockwise
    } else {
        Cube::rotate_face_90_degrees_clockwise
    };

    fn_to_apply(cube, face);
    if token.ends_with(CHAR_FOR_TURN_TWICE) {
        fn_to_apply(cube, face);
    }

    Ok(())
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

    #[test]
    #[should_panic]
    fn test_apply_token_invalid_input() {
        let invalid_token = "M";
        let mut cube = Cube::create(3);
        apply_token(invalid_token, &mut cube).unwrap();
    }

    macro_rules! test_invalid_token {
        ($($name:ident: $value:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut cube = Cube::create(3);
                    let expected_error_msg = format!("Unsupported token in notation string: [{}]", $value);
                    assert_eq!(Err(expected_error_msg), perform_3x3_sequence($value, &mut cube));
                }
            )*
        }
    }

    macro_rules! test_invalid_sequence {
        ($($name:ident: $value:expr, $err_token:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut cube = Cube::create(3);
                    let expected_error_msg = format!("Unsupported token in notation string: [{}]", $err_token);
                    assert_eq!(Err(expected_error_msg), perform_3x3_sequence($value, &mut cube));
                }
            )*
        }
    }

    test_invalid_token!(
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
    fn test_perform_3x3_sequence() {
        let mut cube_under_test = Cube::create(3);
        let mut control_cube = Cube::create(3);

        perform_3x3_sequence("F2 R U' F", &mut cube_under_test)
            .expect("Sequence in test should be valid");

        control_cube.rotate_face_90_degrees_clockwise(Face::Front);
        control_cube.rotate_face_90_degrees_clockwise(Face::Front);
        control_cube.rotate_face_90_degrees_clockwise(Face::Right);
        control_cube.rotate_face_90_degrees_anticlockwise(Face::Up);
        control_cube.rotate_face_90_degrees_clockwise(Face::Front);

        assert_eq!(control_cube, cube_under_test);
    }

    #[test]
    fn test_perform_3x3_sequence_every_token_once() {
        let sequence = "F R U L B D F2 R2 U2 L2 B2 D2 F' R' U' L' B' D'";
        let mut cube_under_test = Cube::create(3);

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
