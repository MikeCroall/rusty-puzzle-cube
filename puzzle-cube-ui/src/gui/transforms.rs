use std::f32::consts::PI;

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
    Mat4::from_scale(0.9 / side_length)
}

pub(super) fn scale_down_inner_cube() -> Matrix4<f32> {
    Mat4::from_scale(0.9999)
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

        #[rustfmt::skip]
        let expected = Matrix4::new(
            0.9999, 0., 0., 0.,
            0., 0.9999, 0., 0.,
            0., 0., 0.9999, 0.,
            0., 0., 0., 1.,
        );

        assert_eq!(expected, actual);
    }
}
