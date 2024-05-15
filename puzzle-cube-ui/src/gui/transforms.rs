use std::f32::consts::PI;

use rusty_puzzle_cube::cube::face::Face;
use three_d::{radians, vec3, Mat4, Matrix4, Rad, Vector3};

const QUARTER_TURN: Rad<f32> = radians(0.5 * PI);
const HALF_TURN: Rad<f32> = radians(PI);
const TRANSLATE_UP: Vector3<f32> = vec3(0., 1., 0.);
const TRANSLATE_TOWARD: Vector3<f32> = vec3(0., 0., 1.);
const TRANSLATE_RIGHT: Vector3<f32> = vec3(1., 0., 0.);

pub(super) fn quarter_turn_around_x() -> Matrix4<f32> {
    Mat4::from_angle_x(QUARTER_TURN)
}

pub(super) fn rev_quarter_turn_around_x() -> Matrix4<f32> {
    Mat4::from_angle_x(-QUARTER_TURN)
}

pub(super) fn quarter_turn_around_y() -> Matrix4<f32> {
    Mat4::from_angle_y(QUARTER_TURN)
}

pub(super) fn rev_quarter_turn_around_y() -> Matrix4<f32> {
    Mat4::from_angle_y(-QUARTER_TURN)
}

pub(super) fn half_turn_around_y() -> Matrix4<f32> {
    Mat4::from_angle_y(HALF_TURN)
}

pub(super) fn translate_up() -> Matrix4<f32> {
    Mat4::from_translation(TRANSLATE_UP)
}

pub(super) fn translate_down() -> Matrix4<f32> {
    Mat4::from_translation(-TRANSLATE_UP)
}

pub(super) fn translate_right() -> Matrix4<f32> {
    Mat4::from_translation(TRANSLATE_RIGHT)
}

pub(super) fn translate_left() -> Matrix4<f32> {
    Mat4::from_translation(-TRANSLATE_RIGHT)
}

pub(super) fn translate_toward() -> Matrix4<f32> {
    Mat4::from_translation(TRANSLATE_TOWARD)
}

pub(super) fn translate_away() -> Matrix4<f32> {
    Mat4::from_translation(-TRANSLATE_TOWARD)
}

pub(super) fn scale_down(side_length: f32) -> Matrix4<f32> {
    let scale = 0.9 / side_length;
    Mat4::from_nonuniform_scale(scale, scale, 0.015)
}

pub(super) fn scale_down_inner_cube() -> Matrix4<f32> {
    #[cfg(not(target_arch = "wasm32"))]
    let scale = Mat4::from_scale(0.9999);

    #[cfg(target_arch = "wasm32")]
    let scale = Mat4::from_scale(0.996);

    scale
}

pub(super) fn position_from_origin_centered_to(side_length: f32, x: f32, y: f32) -> Matrix4<f32> {
    // dist_to_edge is simplified version of (side_length / 2_f32 - 0.5) * 2_f32 / side_length
    let dist_to_edge = 1_f32 - (1_f32 / side_length);
    // three_d in-built square mesh spans from -1.0 to 1.0, so we divide 2 by the amount of tiles to fit
    let scaled_side_length = 2_f32 / side_length;
    let horizontal = TRANSLATE_RIGHT * ((scaled_side_length * x) - dist_to_edge);
    let vertical = TRANSLATE_UP * (dist_to_edge - (scaled_side_length * y));
    Mat4::from_translation(horizontal + vertical)
}

pub(super) fn move_face_into_place(face: Face) -> Matrix4<f32> {
    match face {
        Face::Up => translate_up() * rev_quarter_turn_around_x(),
        Face::Down => translate_down() * quarter_turn_around_x(),
        Face::Front => translate_toward(),
        Face::Right => translate_right() * quarter_turn_around_y(),
        Face::Back => translate_away() * half_turn_around_y(),
        Face::Left => translate_left() * rev_quarter_turn_around_y(),
    }
}

#[allow(clippy::cast_precision_loss)]
pub(super) fn cubie_face_to_transformation(
    side_length: usize,
    face: Face,
    x: usize,
    y: usize,
) -> Matrix4<f32> {
    move_face_into_place(face)
        * position_from_origin_centered_to(side_length as f32, x as f32, y as f32)
        * scale_down(side_length as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use three_d::{Angle as _, Deg, Vector4};

    fn assert_mat_eq_with_tolerance(m1: Matrix4<f32>, m2: Matrix4<f32>, epsilon: f32) {
        assert_vec_eq_with_tolerance(m1.w, m2.w, epsilon);
        assert_vec_eq_with_tolerance(m1.x, m2.x, epsilon);
        assert_vec_eq_with_tolerance(m1.y, m2.y, epsilon);
        assert_vec_eq_with_tolerance(m1.z, m2.z, epsilon);
    }

    fn assert_vec_eq_with_tolerance(v1: Vector4<f32>, v2: Vector4<f32>, epsilon: f32) {
        assert_eq_with_tolerance(v1.w, v2.w, epsilon);
        assert_eq_with_tolerance(v1.x, v2.x, epsilon);
        assert_eq_with_tolerance(v1.y, v2.y, epsilon);
        assert_eq_with_tolerance(v1.z, v2.z, epsilon);
    }

    fn assert_eq_with_tolerance(f1: f32, f2: f32, epsilon: f32) {
        let diff = f1 - f2;
        let abs = diff.abs();
        assert!(abs < epsilon);
    }

    #[test]
    fn test_quarter_turn_around_x() {
        let actual = quarter_turn_around_x();

        let (s, c) = Rad::sin_cos(Deg(90.).into());
        #[rustfmt::skip]
        let expected = Matrix4::new(
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

        let (s, c) = Rad::sin_cos(Deg(-90.).into());
        #[rustfmt::skip]
        let expected = Matrix4::new(
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

        let (s, c) = Rad::sin_cos(Deg(90.).into());
        #[rustfmt::skip]
        let expected = Matrix4::new(
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

        let (s, c) = Rad::sin_cos(Deg(-90.).into());
        #[rustfmt::skip]
        let expected = Matrix4::new(
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

        let (s, c) = Rad::sin_cos(Deg(180.).into());
        #[rustfmt::skip]
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., -1., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scale_down() {
        let actual = scale_down(2.);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            0.45, 0., 0., 0.,
            0., 0.45, 0., 0.,
            0., 0., 0.45, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scale_down_inner_cube() {
        let actual = scale_down_inner_cube();

        #[cfg(not(target_arch = "wasm32"))]
        let expected_scale = 0.9999;

        #[cfg(target_arch = "wasm32")]
        let expected_scale = 0.996;

        #[rustfmt::skip]
        let expected = Matrix4::new(
            expected_scale, 0., 0., 0.,
            0., expected_scale, 0., 0.,
            0., 0., expected_scale, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_position_from_origin_centered_to_1x1_0_0() {
        let actual = position_from_origin_centered_to(1., 0., 0.);

        #[rustfmt::skip]
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            -0.6666666, 0.6666666, 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_position_from_origin_centered_to_3x3_2_0() {
        let actual = position_from_origin_centered_to(3., 2., 0.);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0.66666675, 0.6666666, 0., 1.,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_move_face_into_place_up() {
        let actual = move_face_into_place(Face::Up);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            1., 0., 0., 0.,
            0., 0., -1., 0.,
            0., 1., 0., 0.,
            0., 1., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual, 0.0000001);
    }

    #[test]
    fn test_move_face_into_place_down() {
        let actual = move_face_into_place(Face::Down);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            1., 0., 0., 0.,
            0., 0., 1., 0.,
            0., -1., 0., 0.,
            0., -1., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual, 0.0000001);
    }

    #[test]
    fn test_move_face_into_place_front() {
        let actual = move_face_into_place(Face::Front);

        #[rustfmt::skip]
        let expected = Matrix4::new(
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
        let expected = Matrix4::new(
            -4.371139e-8, 0., -1., 0.,
            0., 1., 0., 0.,
            1., 0., 0., 0.,
            1., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual, 0.0000001);
    }

    #[test]
    fn test_move_face_into_place_back() {
        let actual = move_face_into_place(Face::Back);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            -1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., -1., 0.,
            0., 0., -1., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual, 0.0000001);
    }

    #[test]
    fn test_move_face_into_place_left() {
        let actual = move_face_into_place(Face::Left);

        #[rustfmt::skip]
        let expected = Matrix4::new(
            0., 0., 1., 0.,
            0., 1., 0., 0.,
            -1., 0., 0., 0.,
            -1., 0., 0., 1.,
        );

        assert_mat_eq_with_tolerance(expected, actual, 0.0000001);
    }
}
