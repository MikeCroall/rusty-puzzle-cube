use std::f32::consts::PI;

use rusty_puzzle_cube::cube::{face::Face, Cube};
use three_d::{
    pick, radians, Camera, ColorMaterial, Context, Event, Gm, InnerSpace, Mesh, MouseButton,
    OrbitControl, Rad, Transform, Vec3, Vector3,
};
use tracing::{info, warn};

use crate::gui::transforms::move_face_into_place;

const DIAGONAL_MOVE_THRESHOLD: Rad<f32> = radians(0.125 * PI);

pub(super) struct MouseControl {
    orbit: OrbitControl,
    drag: Option<FaceDrag>,
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

    pub(super) fn handle_events(
        &mut self,
        ctx: &Context,
        inner_cube: &Gm<Mesh, ColorMaterial>,
        side_length: usize,
        camera: &mut Camera,
        events: &mut [Event],
        cube: &mut Cube,
    ) -> bool {
        // todo mouse control cube, or camera orbit if cube itself not interacted

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
                    let face = pick_to_face(start_pick);
                    self.drag = Some(FaceDrag { start_pick, face });
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
                    let new_face = pick_to_face(pick);
                    if face != new_face {
                        self.drag = None;
                        warn!("Moved face from {face:?} to {new_face:?}, cancelling drag");
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
                    let Some(pick) = pick(ctx, camera, *position, inner_cube) else {
                        continue;
                    };
                    displacement_to_move(side_length, *start_pick, pick, *face, cube);
                    *handled = true;
                }
                _ => {}
            }
        }

        self.orbit.handle_events(camera, events)
    }
}

fn pick_to_face(pick: Vector3<f32>) -> Face {
    const EPSILON: f32 = 0.0001;
    if (pick.x - 1.).abs() < EPSILON {
        Face::Right
    } else if (pick.x + 1.).abs() < EPSILON {
        Face::Left
    } else if (pick.y - 1.).abs() < EPSILON {
        Face::Up
    } else if (pick.y + 1.).abs() < EPSILON {
        Face::Down
    } else if (pick.z - 1.).abs() < EPSILON {
        Face::Front
    } else if (pick.z + 1.).abs() < EPSILON {
        Face::Back
    } else {
        panic!("pick_to_face interaction found no valid face from pick. This should never happen with inner cube.");
    }
}

fn displacement_to_move(
    side_length: usize,
    start_pick: Vector3<f32>,
    end_pick: Vector3<f32>,
    face: Face,
    _cube: &mut Cube,
) {
    let Some(unrotate_about_origin) = move_face_into_place(face).inverse_transform() else {
        return;
    };
    let end_pick_unrotated = unrotate_about_origin * end_pick.extend(1.);
    let start_pick_unrotated = unrotate_about_origin * start_pick.extend(1.);
    let unrotated_displacement = (end_pick_unrotated - start_pick_unrotated).truncate();
    info!("Found displacement across face {face:?} to be {unrotated_displacement:?}");

    let angle_to_x = unrotated_displacement.angle(Vector3::unit_x()).0.abs();
    let angle_to_neg_x = unrotated_displacement.angle(-Vector3::unit_x()).0.abs();
    let angle_to_y = unrotated_displacement.angle(Vector3::unit_y()).0.abs();
    let angle_to_neg_y = unrotated_displacement.angle(-Vector3::unit_y()).0.abs();

    let mut angles = [angle_to_x, angle_to_neg_x, angle_to_y, angle_to_neg_y];
    angles.sort_by(|a, b| a.partial_cmp(b).expect("No NaNs here"));

    dbg!(angle_to_x);
    dbg!(angle_to_neg_x);
    dbg!(angle_to_y);
    dbg!(angle_to_neg_y);

    if (angles[0] - angles[1]).abs() < DIAGONAL_MOVE_THRESHOLD.0 {
        warn!("Move was diagonal, skipping...");
        return;
    }

    let smallest = angles[0];
    let move_along_x = smallest == angle_to_x || smallest == angle_to_neg_x;
    let move_to_positive = smallest == angle_to_x || smallest == angle_to_y;
    dbg!(move_along_x);
    dbg!(move_to_positive);

    if move_along_x {
        let row_0_to_1 = (start_pick.y + 1.) / 2.;
        let row = (row_0_to_1 * side_length as f32) as usize;
        dbg!(row);
    } else {
        let col_0_to_1 = (start_pick.x + 1.) / 2.;
        let col = (col_0_to_1 * side_length as f32) as usize;
        dbg!(col);
    }

    todo!("map combination of move_along_x, move_to_positive, face, and row/col to actual fn to mutate cube");
}
