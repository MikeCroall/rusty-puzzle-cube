use std::f32::consts::PI;

use three_d::{radians, vec3, Mat4, Matrix4, Rad, Vector3};

const QUARTER_TURN: Rad<f32> = radians(0.5 * PI);
const HALF_TURN: Rad<f32> = radians(PI);
const TRANSLATE_UP: Vector3<f32> = vec3(0., 1.001, 0.);
const TRANSLATE_TOWARD: Vector3<f32> = vec3(0., 0., 1.001);
const TRANSLATE_RIGHT: Vector3<f32> = vec3(1.001, 0., 0.);

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

pub(super) fn position_from_origin_centered_to(side_length: f32, x: f32, y: f32) -> Matrix4<f32> {
    // dist_to_edge is simplified version of (side_length / 2_f32 - 0.5) * 2_f32 / side_length
    let dist_to_edge = 1_f32 - (1_f32 / side_length);

    let move_to_left = Mat4::from_translation(-TRANSLATE_RIGHT * dist_to_edge);
    let move_to_top = Mat4::from_translation(TRANSLATE_UP * dist_to_edge);
    let move_to_top_left = move_to_top * move_to_left;

    let scaled_side_length = 2_f32 / side_length;
    let move_to_x = TRANSLATE_RIGHT * scaled_side_length * x;
    let move_to_y = -TRANSLATE_UP * scaled_side_length * y;
    let move_from_top_left_to_x_y =
        Mat4::from_translation(move_to_x) * Mat4::from_translation(move_to_y);

    move_from_top_left_to_x_y * move_to_top_left
}
