use crate::cube::{face::Face, Cube};
use once_cell::sync::Lazy;
use regex::Regex;

// TODO make the *_2 variants simply run the normal one twice without having to define a const specifically for the double turns - best syntax for this?
const FN_FOR_TOKEN_F: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Front);
const FN_FOR_TOKEN_F_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_F(c);
    FN_FOR_TOKEN_F(c);
};
const FN_FOR_TOKEN_R: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Right);
const FN_FOR_TOKEN_R_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_R(c);
    FN_FOR_TOKEN_R(c);
};
const FN_FOR_TOKEN_U: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Top);
const FN_FOR_TOKEN_U_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_U(c);
    FN_FOR_TOKEN_U(c);
};
const FN_FOR_TOKEN_L: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Left);
const FN_FOR_TOKEN_L_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_L(c);
    FN_FOR_TOKEN_L(c);
};
const FN_FOR_TOKEN_B: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Back);
const FN_FOR_TOKEN_B_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_B(c);
    FN_FOR_TOKEN_B(c);
};
const FN_FOR_TOKEN_D: fn(&mut Cube) = |c| c.rotate_face_90_degrees_clockwise(Face::Bottom);
const FN_FOR_TOKEN_D_2: fn(&mut Cube) = |c| {
    FN_FOR_TOKEN_D(c);
    FN_FOR_TOKEN_D(c);
};
const FN_FOR_TOKEN_F_PRIME: fn(&mut Cube) = |c| c.rotate_face_90_degrees_anticlockwise(Face::Front);
const FN_FOR_TOKEN_R_PRIME: fn(&mut Cube) = |c| c.rotate_face_90_degrees_anticlockwise(Face::Right);
const FN_FOR_TOKEN_U_PRIME: fn(&mut Cube) = |c| c.rotate_face_90_degrees_anticlockwise(Face::Top);
const FN_FOR_TOKEN_L_PRIME: fn(&mut Cube) = |c| c.rotate_face_90_degrees_anticlockwise(Face::Left);
const FN_FOR_TOKEN_B_PRIME: fn(&mut Cube) = |c| c.rotate_face_90_degrees_anticlockwise(Face::Back);
const FN_FOR_TOKEN_D_PRIME: fn(&mut Cube) =
    |c| c.rotate_face_90_degrees_anticlockwise(Face::Bottom);

static MULTI_TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([FRULDB])(2|')?(\s([FRULDB])(2|')?)*$")
        .expect("Invalid regular expression string in lazy regex")
});

pub(crate) fn perform_3x3_sequence(token_sequence: &str, cube: &mut Cube) {
    let token_sequence = token_sequence.trim();
    assert!(MULTI_TOKEN_REGEX.is_match(token_sequence));

    token_sequence.trim().split(' ').for_each(|token| {
        let fn_for_token = get_fn_for_token(token);
        fn_for_token(cube);
    });
}

fn get_fn_for_token(token: &str) -> fn(&mut Cube) {
    let token = token.trim();

    match token {
        "F" => FN_FOR_TOKEN_F,
        "R" => FN_FOR_TOKEN_R,
        "U" => FN_FOR_TOKEN_U,
        "L" => FN_FOR_TOKEN_L,
        "B" => FN_FOR_TOKEN_B,
        "D" => FN_FOR_TOKEN_D,
        "F2" => FN_FOR_TOKEN_F_2,
        "R2" => FN_FOR_TOKEN_R_2,
        "U2" => FN_FOR_TOKEN_U_2,
        "L2" => FN_FOR_TOKEN_L_2,
        "B2" => FN_FOR_TOKEN_B_2,
        "D2" => FN_FOR_TOKEN_D_2,
        "F'" => FN_FOR_TOKEN_F_PRIME,
        "R'" => FN_FOR_TOKEN_R_PRIME,
        "U'" => FN_FOR_TOKEN_U_PRIME,
        "L'" => FN_FOR_TOKEN_L_PRIME,
        "B'" => FN_FOR_TOKEN_B_PRIME,
        "D'" => FN_FOR_TOKEN_D_PRIME,
        _ => panic!("Unsupported token in notation string: {token}. Regexes should have prevented getting to this point."),
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
    fn test_get_fn_for_token_invalid_input() {
        let invalid_token = "M";
        get_fn_for_token(invalid_token);
    }

    macro_rules! test_multi_token_regex {
        ($expected:literal, $($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($expected, MULTI_TOKEN_REGEX.is_match($value));
                }
            )*
        }
    }

    test_multi_token_regex!(
        true,
        multi_token_basic_matches_f: "F",
        multi_token_basic_matches_r: "R",
        multi_token_basic_matches_u: "U",
        multi_token_basic_matches_l: "L",
        multi_token_basic_matches_d: "D",
        multi_token_basic_matches_b: "B",
        multi_token_basic_matches_f_prime: "F'",
        multi_token_basic_matches_r_prime: "R'",
        multi_token_basic_matches_u_prime: "U'",
        multi_token_basic_matches_l_prime: "L'",
        multi_token_basic_matches_d_prime: "D'",
        multi_token_basic_matches_b_prime: "B'",
        multi_token_basic_matches_f_2: "F2",
        multi_token_basic_matches_r_2: "R2",
        multi_token_basic_matches_u_2: "U2",
        multi_token_basic_matches_l_2: "L2",
        multi_token_basic_matches_d_2: "D2",
        multi_token_basic_matches_b_2: "B2",
        multi_token_basic_matches: "F R U L D B",
        multi_token_basic_numbers_matches: "F R2 U2 L D2 B",
        multi_token_basic_primes_matches: "F' R U L' D B'",
        multi_token_basic_primes_and_numbers_matches: "F' R2 U2 L' D2 B'",
        multi_token_basic_repeats_matches: "F2 U2 F2 U2 F' U' F' U'",
    );

    test_multi_token_regex!(
        false,
        multi_token_does_not_match_f_0: "F0",
        multi_token_does_not_match_f_1: "F1",
        multi_token_does_not_match_f_1_prime: "F1'",
        multi_token_does_not_match_f_2_prime: "F2'",
        multi_token_does_not_match_f_prime_1: "F'1",
        multi_token_does_not_match_f_prime_2: "F'2",
        multi_token_does_not_match_f_3: "F3",
        multi_token_does_not_match_f_f: "FF",
        multi_token_does_not_match_f_f_1: "FF1",
        multi_token_does_not_match_f_f_2: "FF2",
        multi_token_does_not_match_f_2_2: "F22",
        multi_token_does_not_match_1: "1",
        multi_token_does_not_match_2: "2",
        multi_token_does_not_match_3: "3",
        multi_token_does_not_match_too_many_spaces: "F  R U",
        multi_token_does_not_match_not_enough_spaces: "FR U",
        multi_token_does_not_match_invalid_individual_tokens: "F2' R'' UU",
        multi_token_does_not_match_invalid_char: "F2 R G U",
        multi_token_does_not_match_invalid_chars: "F2_ R@ UU",
    );

    #[test]
    fn test_perform_3x3_sequence() {
        let mut cube_under_test = Cube::create(3);
        let mut control_cube = Cube::create(3);

        perform_3x3_sequence("F2 R U F", &mut cube_under_test);

        control_cube.rotate_face_90_degrees_clockwise(Face::Front);
        control_cube.rotate_face_90_degrees_clockwise(Face::Front);
        control_cube.rotate_face_90_degrees_clockwise(Face::Right);
        control_cube.rotate_face_90_degrees_clockwise(Face::Top);
        control_cube.rotate_face_90_degrees_clockwise(Face::Front);

        assert_eq!(control_cube, cube_under_test);
    }

    #[test]
    fn test_perform_3x3_sequence_every_token_once() {
        let sequence = "F R U L B D F2 R2 U2 L2 B2 D2 F' R' U' L' B' D'";
        let mut cube_under_test = Cube::create(3);

        perform_3x3_sequence(sequence, &mut cube_under_test);

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
