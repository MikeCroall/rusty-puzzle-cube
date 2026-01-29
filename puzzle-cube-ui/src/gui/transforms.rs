use std::f32::consts::PI;

use rusty_puzzle_cube::cube::face::Face;
use three_d::{Mat4, Radians, Vec3, radians, vec3};

pub const QUARTER_TURN: Radians = radians(0.5 * PI);
const HALF_TURN: Radians = radians(PI);
const TRANSLATE_UP: Vec3 = vec3(0., 1., 0.);
const TRANSLATE_TOWARD: Vec3 = vec3(0., 0., 1.);
const TRANSLATE_RIGHT: Vec3 = vec3(1., 0., 0.);

pub(super) fn fraction_of_quarter_turn(fraction: f32) -> Radians {
    radians(fraction * QUARTER_TURN.0)
}

pub(super) fn quarter_turn_around_x() -> Mat4 {
    Mat4::from_angle_x(QUARTER_TURN)
}

pub(super) fn rev_quarter_turn_around_x() -> Mat4 {
    Mat4::from_angle_x(-QUARTER_TURN)
}

pub(super) fn quarter_turn_around_y() -> Mat4 {
    Mat4::from_angle_y(QUARTER_TURN)
}

pub(super) fn rev_quarter_turn_around_y() -> Mat4 {
    Mat4::from_angle_y(-QUARTER_TURN)
}

pub(super) fn half_turn_around_y() -> Mat4 {
    Mat4::from_angle_y(HALF_TURN)
}

pub(super) fn translate_up() -> Mat4 {
    Mat4::from_translation(TRANSLATE_UP)
}

pub(super) fn translate_down() -> Mat4 {
    Mat4::from_translation(-TRANSLATE_UP)
}

pub(super) fn translate_right() -> Mat4 {
    Mat4::from_translation(TRANSLATE_RIGHT)
}

pub(super) fn translate_left() -> Mat4 {
    Mat4::from_translation(-TRANSLATE_RIGHT)
}

pub(super) fn translate_toward() -> Mat4 {
    Mat4::from_translation(TRANSLATE_TOWARD)
}

pub(super) fn translate_away() -> Mat4 {
    Mat4::from_translation(-TRANSLATE_TOWARD)
}

pub(super) fn recess_backing(side_length: f32) -> Mat4 {
    Mat4::from_translation(-TRANSLATE_TOWARD * 1. / side_length)
}

pub(super) fn scale_backing(side_length: f32) -> Mat4 {
    let scale = 1. / side_length;
    Mat4::from_nonuniform_scale(scale, scale, scale)
}

pub(super) fn scale_down(side_length: f32) -> Mat4 {
    let scale = 0.9 / side_length;
    Mat4::from_nonuniform_scale(scale, scale, 0.015 * 3. / side_length)
}

pub(super) fn position_from_origin_centered_to(side_length: f32, x: f32, y: f32) -> Mat4 {
    // dist_to_edge is simplified version of (side_length / 2_f32 - 0.5) * 2_f32 / side_length
    let dist_to_edge = 1_f32 - (1_f32 / side_length);
    // three_d in-built square mesh spans from -1.0 to 1.0, so we divide 2 by the amount of tiles to fit
    let scaled_side_length = 2_f32 / side_length;
    let horizontal = TRANSLATE_RIGHT * ((scaled_side_length * x) - dist_to_edge);
    let vertical = TRANSLATE_UP * (dist_to_edge - (scaled_side_length * y));
    Mat4::from_translation(horizontal + vertical)
}

pub(super) fn move_face_into_place(face: Face) -> Mat4 {
    match face {
        Face::Up => translate_up() * rev_quarter_turn_around_x(),
        Face::Down => translate_down() * quarter_turn_around_x(),
        Face::Front => translate_toward(),
        Face::Right => translate_right() * quarter_turn_around_y(),
        Face::Back => translate_away() * half_turn_around_y(),
        Face::Left => translate_left() * rev_quarter_turn_around_y(),
    }
}

#[expect(clippy::cast_precision_loss)]
pub(super) fn cubie_face_to_backing_transformation(
    side_length: usize,
    face: Face,
    x: usize,
    y: usize,
) -> Mat4 {
    move_face_into_place(face)
        * recess_backing(side_length as f32)
        * position_from_origin_centered_to(side_length as f32, x as f32, y as f32)
        * scale_backing(side_length as f32)
}

#[expect(clippy::cast_precision_loss)]
pub(super) fn cubie_face_to_transformation(
    side_length: usize,
    face: Face,
    x: usize,
    y: usize,
) -> Mat4 {
    move_face_into_place(face)
        * position_from_origin_centered_to(side_length as f32, x as f32, y as f32)
        * scale_down(side_length as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use three_d::{Angle as _, Deg, Vec4};

    fn assert_mat_eq_with_tolerance(m1: Mat4, m2: Mat4) {
        assert_vec_eq_with_tolerance(m1.w, m2.w);
        assert_vec_eq_with_tolerance(m1.x, m2.x);
        assert_vec_eq_with_tolerance(m1.y, m2.y);
        assert_vec_eq_with_tolerance(m1.z, m2.z);
    }

    fn assert_vec_eq_with_tolerance(v1: Vec4, v2: Vec4) {
        assert_eq_with_tolerance(v1.w, v2.w);
        assert_eq_with_tolerance(v1.x, v2.x);
        assert_eq_with_tolerance(v1.y, v2.y);
        assert_eq_with_tolerance(v1.z, v2.z);
    }

    fn assert_eq_with_tolerance(f1: f32, f2: f32) {
        let diff = f1 - f2;
        let abs = diff.abs();
        assert!(abs < f32::EPSILON);
    }

    #[test]
    fn test_fraction_of_quarter_turn() {
        assert_eq!(radians(0.45 * PI), fraction_of_quarter_turn(0.9));
        assert_eq!(radians(0.25 * PI), fraction_of_quarter_turn(0.5));
        assert_eq!(radians(0.05 * PI), fraction_of_quarter_turn(0.1));
    }

    #[test]
    fn test_quarter_turn_around_x() {
        let actual = quarter_turn_around_x();

        let (s, c) = Radians::sin_cos(Deg(90.).into());
        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., c, s, 0.,
            0., -s, c, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rev_quarter_turn_around_x() {
        let actual = rev_quarter_turn_around_x();

        let (s, c) = Radians::sin_cos(Deg(-90.).into());
        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., c, s, 0.,
            0., -s, c, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_quarter_turn_around_y() {
        let actual = quarter_turn_around_y();

        let (s, c) = Radians::sin_cos(Deg(90.).into());
        #[rustfmt::skip]
        let expected = Mat4::new(
            c, 0., -s, 0.,
            0., 1., 0., 0.,
            s, 0., c, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rev_quarter_turn_around_y() {
        let actual = rev_quarter_turn_around_y();

        let (s, c) = Radians::sin_cos(Deg(-90.).into());
        #[rustfmt::skip]
        let expected = Mat4::new(
            c, 0., -s, 0.,
            0., 1., 0., 0.,
            s, 0., c, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_half_turn_around_y() {
        let actual = half_turn_around_y();

        let (s, c) = Radians::sin_cos(Deg(180.).into());
        #[rustfmt::skip]
        let expected = Mat4::new(
            c, 0., -s, 0.,
            0., 1., 0., 0.,
            s, 0., c, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_up() {
        let actual = translate_up();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 1., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_down() {
        let actual = translate_down();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., -1., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_right() {
        let actual = translate_right();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            1., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_left() {
        let actual = translate_left();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            -1., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_toward() {
        let actual = translate_toward();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 1., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_translate_away() {
        let actual = translate_away();

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., -1., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_recess_backing_small_side_length() {
        let actual = recess_backing(2.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., -0.5, 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_scale_backing_small_side_length() {
        let actual = scale_backing(2.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0.5, 0., 0., 0.,
            0., 0.5, 0., 0.,
            0., 0., 0.5, 0.,
            0., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_scale_down_small_side_length() {
        let actual = scale_down(2.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0.45, 0., 0., 0.,
            0., 0.45, 0., 0.,
            0., 0., 0.0225, 0.,
            0., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_scale_down_large_side_length() {
        let actual = scale_down(30.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0.03, 0., 0., 0.,
            0., 0.03, 0., 0.,
            0., 0., 0.0015, 0.,
            0., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_position_from_origin_centered_to_1x1_0_0() {
        let actual = position_from_origin_centered_to(1., 0., 0.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_position_from_origin_centered_to_3x3_0_0() {
        let actual = position_from_origin_centered_to(3., 0., 0.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            -0.666_666_6, 0.666_666_6, 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_position_from_origin_centered_to_3x3_2_0() {
        let actual = position_from_origin_centered_to(3., 2., 0.);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0.666_666_75, 0.666_666_6, 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_up() {
        let actual = move_face_into_place(Face::Up);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 0., -1., 0.,
            0., 1., 0., 0.,
            0., 1., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_down() {
        let actual = move_face_into_place(Face::Down);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 0., 1., 0.,
            0., -1., 0., 0.,
            0., -1., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_front() {
        let actual = move_face_into_place(Face::Front);

        #[rustfmt::skip]
        let expected = Mat4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 1., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_right() {
        let actual = move_face_into_place(Face::Right);

        #[rustfmt::skip]
        let expected = Mat4::new(
            -4.371_139e-8, 0., -1., 0.,
            0., 1., 0., 0.,
            1., 0., 0., 0.,
            1., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_back() {
        let actual = move_face_into_place(Face::Back);

        #[rustfmt::skip]
        let expected = Mat4::new(
            -1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., -1., 0.,
            0., 0., -1., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_left() {
        let actual = move_face_into_place(Face::Left);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0., 0., 1., 0.,
            0., 1., 0., 0.,
            -1., 0., 0., 0.,
            -1., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_cubie_face_to_backing_transformation_example_1() {
        let side_length = 3;
        let face = Face::Front;
        let x = 0;
        let y = 1;

        let actual = cubie_face_to_backing_transformation(side_length, face, x, y);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0.33333334, 0., 0., 0.,
            0., 0.33333334, 0., 0.,
            0., 0., 0.33333334, 0.,
            -0.6666666, 0., 0.6666666, 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_cubie_face_to_backing_transformation_example_2() {
        let side_length = 10;
        let face = Face::Right;
        let x = 7;
        let y = 4;

        let actual = cubie_face_to_backing_transformation(side_length, face, x, y);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0., 0., -0.1, 0.,
            0., 0.1, 0., 0.,
            0.1, 0., 0., 0.,
            0.9, 0.1, -0.5, 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_cubie_face_to_transformation_example_1() {
        let side_length = 3;
        let face = Face::Front;
        let x = 0;
        let y = 1;

        let actual = cubie_face_to_transformation(side_length, face, x, y);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0.3, 0., 0., 0.,
            0., 0.3, 0., 0.,
            0., 0., 0.015, 0.,
            -0.6666666, 0., 1., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }

    #[test]
    fn test_cubie_face_to_transformation_example_2() {
        let side_length = 10;
        let face = Face::Right;
        let x = 7;
        let y = 4;

        let actual = cubie_face_to_transformation(side_length, face, x, y);

        #[rustfmt::skip]
        let expected = Mat4::new(
            0., 0., -0.09, 0.,
            0., 0.09, 0., 0.,
            0.0045, 0., 0., 0.,
            1., 0.1, -0.5, 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual);
    }
}
