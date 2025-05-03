use std::f32::consts::PI;

use rusty_puzzle_cube::cube::{PuzzleCube, face::Face};
use three_d::{
    Camera, ColorMaterial, Context, Event, Gm, InnerSpace, Mesh, MouseButton, OrbitControl, Rad,
    Transform, Vec3, Vector3, pick, radians,
};
use tracing::{error, warn};

use crate::gui::{decided_move::DecidedMove, transforms::move_face_into_place};

const MOVE_TOO_SMALL_THRESHOLD: f32 = 0.15;
const DIAGONAL_MOVE_THRESHOLD: Rad<f32> = radians(0.125 * PI);
const EPSILON: f32 = 0.01;

pub(super) struct MouseControl {
    orbit: OrbitControl,
    drag: Option<FaceDrag>,
}

pub(super) struct MouseControlOutput {
    pub(super) redraw: bool,
    pub(super) updated_cube: bool,
}

struct FaceDrag {
    start_pick: Vector3<f32>,
    face: Face,
}

impl MouseControl {
    pub(super) fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            orbit: OrbitControl::new(target, min_distance, max_distance),
            drag: None,
        }
    }

    pub(super) fn handle_events<C: PuzzleCube>(
        &mut self,
        ctx: &Context,
        inner_cube: &Gm<Mesh, ColorMaterial>,
        side_length: usize,
        camera: &mut Camera,
        events: &mut [Event],
        cube: &mut C,
    ) -> MouseControlOutput {
        let mut updated_cube = false;
        for event in events.iter_mut() {
            match event {
                Event::MousePress {
                    button: MouseButton::Left,
                    position,
                    handled,
                    ..
                } => {
                    let Some(start_pick) = pick(ctx, camera, *position, inner_cube) else {
                        continue;
                    };
                    let Some(face) = pick_to_face(start_pick.position) else {
                        continue;
                    };
                    self.drag = Some(FaceDrag {
                        start_pick: start_pick.position,
                        face,
                    });
                    *handled = true;
                }
                Event::MouseMotion {
                    button: Some(MouseButton::Left),
                    position,
                    handled,
                    ..
                } => {
                    let Some(FaceDrag { face, .. }) = self.drag else {
                        continue;
                    };
                    let Some(pick) = pick(ctx, camera, *position, inner_cube) else {
                        continue;
                    };
                    let Some(new_face) = pick_to_face(pick.position) else {
                        continue;
                    };
                    if face != new_face {
                        self.drag = None;
                        warn!("Dragged from face {face:?} to {new_face:?}, skipping...");
                    }
                    *handled = true;
                }
                Event::MouseRelease {
                    button: MouseButton::Left,
                    position,
                    handled,
                    ..
                } => {
                    let Some(FaceDrag { start_pick, face }) = &self.drag else {
                        continue;
                    };
                    let Some(end_pick) = pick(ctx, camera, *position, inner_cube) else {
                        continue;
                    };
                    if let Some(decided_move) =
                        picks_to_move(side_length, *start_pick, end_pick.position, *face)
                    {
                        decided_move.apply(cube);
                        updated_cube = true;
                        *handled = true;
                    }
                }
                _ => {}
            }
        }

        MouseControlOutput {
            updated_cube,
            redraw: updated_cube || self.orbit.handle_events(camera, events),
        }
    }
}

fn pick_to_face(pick: Vector3<f32>) -> Option<Face> {
    if (pick.x - 1.).abs() < EPSILON {
        Some(Face::Right)
    } else if (pick.x + 1.).abs() < EPSILON {
        Some(Face::Left)
    } else if (pick.y - 1.).abs() < EPSILON {
        Some(Face::Up)
    } else if (pick.y + 1.).abs() < EPSILON {
        Some(Face::Down)
    } else if (pick.z - 1.).abs() < EPSILON {
        Some(Face::Front)
    } else if (pick.z + 1.).abs() < EPSILON {
        Some(Face::Back)
    } else {
        error!(
            "pick_to_face interaction found no valid face from pick. This should never happen with inner cube."
        );
        None
    }
}

#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn picks_to_move(
    side_length: usize,
    start_pick: Vector3<f32>,
    end_pick: Vector3<f32>,
    dragged_face: Face,
) -> Option<DecidedMove> {
    let (start_pick, end_pick) = unrotate_picks(start_pick, end_pick, dragged_face);
    let (move_along_x, toward_positive) = validate_straight_dir(start_pick, end_pick)?;

    let (face, clockwise) = if move_along_x {
        let row_0_to_1 = (start_pick.y + 1.) / 2.;
        let row = (row_0_to_1 * side_length as f32) as usize;
        if row != 0 && row != side_length - 1 {
            return Some(DecidedMove::InnerRow {
                face: dragged_face,
                row,
                toward_positive,
            });
        }
        translate_horizontal_drag(row, dragged_face, toward_positive)
    } else {
        let col_0_to_1 = (start_pick.x + 1.) / 2.;
        let col = (col_0_to_1 * side_length as f32) as usize;
        if col != 0 && col != side_length - 1 {
            return Some(DecidedMove::InnerCol {
                face: dragged_face,
                col,
                toward_positive,
            });
        }
        translate_vertical_drag(col, dragged_face, toward_positive)
    };
    Some(DecidedMove::WholeFace { face, clockwise })
}

fn unrotate_picks(
    start_pick: Vector3<f32>,
    end_pick: Vector3<f32>,
    face: Face,
) -> (Vector3<f32>, Vector3<f32>) {
    let unrotate_mat = move_face_into_place(face)
        .inverse_transform()
        .expect("All faces rotations must be invertible");
    let start_pick = (unrotate_mat * start_pick.extend(1.)).truncate();
    let end_pick = (unrotate_mat * end_pick.extend(1.)).truncate();
    (start_pick, end_pick)
}

fn validate_straight_dir(
    unrotated_start_pick: Vector3<f32>,
    unrotated_end_pick: Vector3<f32>,
) -> Option<(bool, bool)> {
    let displacement = unrotated_end_pick - unrotated_start_pick;
    if displacement.magnitude() < MOVE_TOO_SMALL_THRESHOLD {
        warn!("Move was too small, skipping...");
        return None;
    }

    let angle_to_x = displacement.angle(Vector3::unit_x()).0.abs();
    let angle_to_neg_x = displacement.angle(-Vector3::unit_x()).0.abs();
    let angle_to_y = displacement.angle(Vector3::unit_y()).0.abs();
    let angle_to_neg_y = displacement.angle(-Vector3::unit_y()).0.abs();

    let mut angles = [angle_to_x, angle_to_neg_x, angle_to_y, angle_to_neg_y];
    angles.sort_by(|a, b| a.partial_cmp(b).expect("No NaNs here"));

    if (angles[0] - angles[1]).abs() < DIAGONAL_MOVE_THRESHOLD.0 {
        warn!("Move was diagonal, skipping...");
        return None;
    }

    let smallest = angles[0];
    let positive_horizontal = (smallest - angle_to_x).abs() < EPSILON;
    let negative_horizontal = (smallest - angle_to_neg_x).abs() < EPSILON;
    let positive_vertical = (smallest - angle_to_y).abs() < EPSILON;
    let move_along_x = positive_horizontal || negative_horizontal;
    let toward_positive = positive_horizontal || positive_vertical;
    Some((move_along_x, toward_positive))
}

fn translate_vertical_drag(col: usize, dragged_face: Face, toward_positive: bool) -> (Face, bool) {
    let col_0 = col == 0;
    let face = match (dragged_face, col_0) {
        (Face::Up | Face::Down | Face::Front, true) | (Face::Back, false) => Face::Left,
        (Face::Up | Face::Down | Face::Front, false) | (Face::Back, true) => Face::Right,
        (Face::Right, true) | (Face::Left, false) => Face::Front,
        (Face::Right, false) | (Face::Left, true) => Face::Back,
    };
    let clockwise = match (dragged_face, face) {
        (Face::Up | Face::Down | Face::Front, Face::Left)
        | (Face::Right, Face::Front)
        | (Face::Back, Face::Right)
        | (Face::Left, Face::Back) => !toward_positive,
        (Face::Up | Face::Down | Face::Front, Face::Right)
        | (Face::Right, Face::Back)
        | (Face::Back, Face::Left)
        | (Face::Left, Face::Front) => toward_positive,
        _ => unreachable!(),
    };
    (face, clockwise)
}

fn translate_horizontal_drag(
    row: usize,
    dragged_face: Face,
    toward_positive: bool,
) -> (Face, bool) {
    let row_0 = row == 0;
    let face = match (dragged_face, row_0) {
        (Face::Up, true) | (Face::Down, false) => Face::Front,
        (Face::Up, false) | (Face::Down, true) => Face::Back,
        (Face::Front | Face::Right | Face::Back | Face::Left, true) => Face::Down,
        (Face::Front | Face::Right | Face::Back | Face::Left, false) => Face::Up,
    };
    let clockwise = match (dragged_face, face) {
        (Face::Up, Face::Front)
        | (Face::Down, Face::Back)
        | (Face::Front | Face::Right | Face::Back | Face::Left, Face::Down) => toward_positive,
        (Face::Up, Face::Back)
        | (Face::Down, Face::Front)
        | (Face::Front | Face::Right | Face::Back | Face::Left, Face::Up) => !toward_positive,
        _ => unreachable!(),
    };
    (face, clockwise)
}

#[cfg(test)]
mod tests {
    // todo write tests to keep it working!
}
