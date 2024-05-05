use std::f32::consts::PI;

use three_d::{radians, vec3, ColorMaterial, Gm, Mat4, Matrix4, Mesh, Rad, Vector3};

const QUARTER_TURN: Rad<f32> = radians(0.5 * PI);
const TRANSLATE_UP: Vector3<f32> = vec3(0., 1., 0.);
const TRANSLATE_TOWARD: Vector3<f32> = vec3(0., 0., 1.);
const TRANSLATE_RIGHT: Vector3<f32> = vec3(1., 0., 0.);

pub(super) fn quarter_turn_around_x(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_angle_x(QUARTER_TURN));
}

pub(super) fn quarter_turn_around_y(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_angle_y(QUARTER_TURN));
}

pub(super) fn translate_up(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(TRANSLATE_UP));
}

pub(super) fn translate_down(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(-TRANSLATE_UP));
}

pub(super) fn translate_right(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(TRANSLATE_RIGHT));
}

pub(super) fn translate_left(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(-TRANSLATE_RIGHT));
}

pub(super) fn translate_toward(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(TRANSLATE_TOWARD));
}

pub(super) fn translate_away(obj: &mut Gm<Mesh, ColorMaterial>) {
    apply_transform(obj, Mat4::from_translation(-TRANSLATE_TOWARD));
}

fn apply_transform(obj: &mut Gm<Mesh, ColorMaterial>, transform: Matrix4<f32>) {
    let existing_transform = obj.transformation();
    obj.set_transformation(transform * existing_transform);
}
