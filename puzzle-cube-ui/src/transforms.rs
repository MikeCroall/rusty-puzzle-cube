use std::f32::consts::PI;

use three_d::{radians, vec3, Mat4, Matrix4, Rad, Vector3};

const QUARTER_TURN: Rad<f32> = radians(0.5 * PI);
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

pub(super) fn rev_quarter_turn_around_z() -> Matrix4<f32> {
    Mat4::from_angle_z(-QUARTER_TURN)
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

pub(super) fn scale_to_top_left(side_length: f32) -> Matrix4<f32> {
    let scale_factor = 1_f32 / side_length;
    let scale_mat = Mat4::from_scale(scale_factor);

    let dist_to_edge = (side_length / 2_f32 - 0.5) * 2_f32 * scale_factor;
    let move_to_left = Mat4::from_translation(-TRANSLATE_RIGHT * dist_to_edge);
    let move_to_top = Mat4::from_translation(TRANSLATE_UP * dist_to_edge);
    move_to_top * move_to_left * scale_mat
}

pub(super) fn position_to(side_length: f32, x: f32, y: f32) -> Matrix4<f32> {
    let scaled_side_length = 2_f32 / side_length;
    let translate_right = TRANSLATE_RIGHT * scaled_side_length * x;
    let translate_down = -TRANSLATE_UP * scaled_side_length * y;
    Mat4::from_translation(translate_right) * Mat4::from_translation(translate_down)
}

#[macro_export]
macro_rules! combine_transformations {
    ($transform:ident) => {
        $crate::transforms::$transform()
    };
    ($transform:ident, $($tail:ident),+) => {
        combine_transformations!($($tail),*) * $crate::transforms::$transform()
    };
}
