use std::f32::consts::PI;

use crate::gui::{
    GuiState, anim_cube::AnimCube, decided_move::DecidedMove, transforms::move_face_into_place,
};
use rusty_puzzle_cube::cube::{
    Cube, PuzzleCube as _, direction::Direction, face::Face, rotation::Rotation,
};
use three_d::{
    Event, FreeOrbitControl, InnerSpace, MouseButton, OrbitControl, Radians, Transform, Vec3, pick,
    radians,
};
use three_d_asset::PixelPoint;
use tracing::{debug, error};

const MOVE_TOO_SMALL_THRESHOLD: f32 = 0.15;
const DIAGONAL_MOVE_THRESHOLD: Radians = radians(0.125 * PI);
const EPSILON: f32 = 0.01;

pub(super) struct MouseControl {
    free_orbit: FreeOrbitControl,
    upright_orbit: OrbitControl,
    drag: Option<FaceDrag>,
}

pub(super) struct MouseControlOutput {
    pub(super) redraw: bool,
    pub(super) updated_cube: bool,
    pub(super) rotation_if_released_now: RotationIfReleasedNow,
}

#[derive(Copy, Clone, PartialEq)]
pub(super) enum RotationIfReleasedNow {
    NotAttempted,
    Invalid,
    Valid(Rotation),
}

struct FaceDrag {
    start_pick: Vec3,
    face: Face,
}

impl MouseControl {
    pub(super) fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            free_orbit: FreeOrbitControl::new(target, min_distance, max_distance),
            upright_orbit: OrbitControl {
                target,
                min_distance,
                max_distance,
            },
            drag: None,
        }
    }

    pub(super) fn handle_events(
        &mut self,
        state: &mut GuiState<AnimCube<Cube>, 100>,
        events: &mut [Event],
    ) -> MouseControlOutput {
        if events.is_empty() {
            return MouseControlOutput {
                redraw: false,
                updated_cube: false,
                rotation_if_released_now: state.rotation_if_released_now,
            };
        }

        let mut updated_cube = false;
        let mut rotation_if_released_now = RotationIfReleasedNow::NotAttempted;
        for event in events.iter_mut() {
            match event {
                Event::MousePress {
                    button: MouseButton::Left,
                    position,
                    handled,
                    ..
                } => {
                    self.handle_mouse_press_left_btn(state, *position, handled);
                }
                Event::MouseMotion {
                    button: Some(MouseButton::Left),
                    position,
                    handled,
                    ..
                } => {
                    self.handle_mouse_motion_left_btn(
                        state,
                        &mut rotation_if_released_now,
                        *position,
                        handled,
                    );
                }
                Event::MouseRelease {
                    button: MouseButton::Left,
                    position,
                    handled,
                    ..
                } => {
                    self.handle_mouse_release_left_btn(
                        state,
                        *position,
                        handled,
                        &mut updated_cube,
                    );
                }
                _ => {}
            }
        }

        MouseControlOutput {
            updated_cube,
            redraw: updated_cube
                || if state.lock_upright {
                    self.upright_orbit.handle_events(&mut state.camera, events)
                } else {
                    self.free_orbit.handle_events(&mut state.camera, events)
                },
            rotation_if_released_now,
        }
    }

    fn handle_mouse_press_left_btn(
        &mut self,
        state: &mut GuiState<AnimCube<Cube>, 100>,
        position: PixelPoint,
        handled: &mut bool,
    ) {
        let Some(start_pick) = pick(&state.ctx, &state.camera, position, &state.pick_cube) else {
            return;
        };
        let Some(face) = pick_to_face(start_pick.position) else {
            return;
        };
        self.drag = Some(FaceDrag {
            start_pick: start_pick.position,
            face,
        });
        *handled = true;
    }

    fn handle_mouse_motion_left_btn(
        &mut self,
        state: &mut GuiState<AnimCube<Cube>, 100>,
        rotation_if_released_now: &mut RotationIfReleasedNow,
        position: PixelPoint,
        handled: &mut bool,
    ) {
        let Some(FaceDrag { start_pick, face }) = self.drag else {
            return;
        };
        let Some(current_pick) = pick(&state.ctx, &state.camera, position, &state.pick_cube) else {
            return;
        };
        let Some(new_face) = pick_to_face(current_pick.position) else {
            return;
        };
        if face != new_face {
            self.drag = None;
            debug!("Dragged from face {face:?} to {new_face:?}, skipping...");
            *rotation_if_released_now = RotationIfReleasedNow::Invalid;
        } else if let Some(rotation) =
            picks_to_move(state.side_length, start_pick, current_pick.position, face)
                .map(|decided_move| decided_move.as_rotation().normalise(state.side_length))
        {
            *rotation_if_released_now = RotationIfReleasedNow::Valid(rotation);
        } else {
            *rotation_if_released_now = RotationIfReleasedNow::Invalid;
        }
        *handled = true;
    }

    fn handle_mouse_release_left_btn(
        &mut self,
        state: &mut GuiState<AnimCube<Cube>, 100>,
        position: PixelPoint,
        handled: &mut bool,
        updated_cube: &mut bool,
    ) {
        let Some(FaceDrag { start_pick, face }) = &self.drag else {
            return;
        };
        let Some(end_pick) = pick(&state.ctx, &state.camera, position, &state.pick_cube) else {
            return;
        };
        if let Some(decided_move) =
            picks_to_move(state.side_length, *start_pick, end_pick.position, *face)
        {
            let rotation = decided_move.as_rotation();
            match state.cube.rotate(rotation) {
                Ok(()) => {
                    state.undo_queue.push_back(rotation);
                    *updated_cube = true;
                }
                Err(e) => error!("Invalid rotation was provided to cube. {e:?}"),
            }
            *handled = true;
        }
    }
}

fn pick_to_face(pick: Vec3) -> Option<Face> {
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
    start_pick: Vec3,
    end_pick: Vec3,
    dragged_face: Face,
) -> Option<DecidedMove> {
    let UnrotatedPicks {
        start_pick,
        end_pick,
    } = unrotate_picks(start_pick, end_pick, dragged_face);
    let ValidatedStraightDir {
        move_along_x,
        toward_positive,
    } = validate_straight_dir(start_pick, end_pick)?;

    let TranslatedDragForWholeFace { face, direction } = if move_along_x {
        let row_0_to_1 = f32::midpoint(start_pick.y, 1.);
        let row = (row_0_to_1 * side_length as f32) as usize;
        if row != 0 && row != side_length - 1 {
            return Some(DecidedMove::InnerRow {
                face: dragged_face,
                row,
                toward_positive,
            });
        }
        translate_horizontal_drag_for_whole_face(row, dragged_face, toward_positive)
    } else {
        let col_0_to_1 = f32::midpoint(start_pick.x, 1.);
        let col = (col_0_to_1 * side_length as f32) as usize;
        if col != 0 && col != side_length - 1 {
            return Some(DecidedMove::InnerCol {
                face: dragged_face,
                col,
                toward_positive,
            });
        }
        translate_vertical_drag_for_whole_face(col, dragged_face, toward_positive)
    };
    Some(DecidedMove::WholeFace { face, direction })
}

struct UnrotatedPicks {
    start_pick: Vec3,
    end_pick: Vec3,
}

fn unrotate_picks(start_pick: Vec3, end_pick: Vec3, face: Face) -> UnrotatedPicks {
    let unrotate_mat = move_face_into_place(face)
        .inverse_transform()
        .expect("All faces rotations must be invertible");
    let start_pick = (unrotate_mat * start_pick.extend(1.)).truncate();
    let end_pick = (unrotate_mat * end_pick.extend(1.)).truncate();
    UnrotatedPicks {
        start_pick,
        end_pick,
    }
}

struct ValidatedStraightDir {
    move_along_x: bool,
    toward_positive: bool,
}

fn validate_straight_dir(
    unrotated_start_pick: Vec3,
    unrotated_end_pick: Vec3,
) -> Option<ValidatedStraightDir> {
    let displacement = unrotated_end_pick - unrotated_start_pick;
    if displacement.magnitude() < MOVE_TOO_SMALL_THRESHOLD {
        debug!("Move was too small, skipping...");
        return None;
    }

    let angle_to_x = displacement.angle(Vec3::unit_x()).0.abs();
    let angle_to_neg_x = displacement.angle(-Vec3::unit_x()).0.abs();
    let angle_to_y = displacement.angle(Vec3::unit_y()).0.abs();
    let angle_to_neg_y = displacement.angle(-Vec3::unit_y()).0.abs();

    let mut angles = [angle_to_x, angle_to_neg_x, angle_to_y, angle_to_neg_y];
    angles.sort_by(|a, b| a.partial_cmp(b).expect("No NaNs here"));

    if (angles[0] - angles[1]).abs() < DIAGONAL_MOVE_THRESHOLD.0 {
        debug!("Move was diagonal, skipping...");
        return None;
    }

    let smallest = angles[0];
    let positive_horizontal = (smallest - angle_to_x).abs() < EPSILON;
    let negative_horizontal = (smallest - angle_to_neg_x).abs() < EPSILON;
    let positive_vertical = (smallest - angle_to_y).abs() < EPSILON;
    let move_along_x = positive_horizontal || negative_horizontal;
    let toward_positive = positive_horizontal || positive_vertical;
    Some(ValidatedStraightDir {
        move_along_x,
        toward_positive,
    })
}

struct TranslatedDragForWholeFace {
    face: Face,
    direction: Direction,
}

fn translate_vertical_drag_for_whole_face(
    col: usize,
    dragged_face: Face,
    toward_positive: bool,
) -> TranslatedDragForWholeFace {
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
    let direction = if clockwise {
        Direction::Clockwise
    } else {
        Direction::Anticlockwise
    };
    TranslatedDragForWholeFace { face, direction }
}

fn translate_horizontal_drag_for_whole_face(
    row: usize,
    dragged_face: Face,
    toward_positive: bool,
) -> TranslatedDragForWholeFace {
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
    let direction = if clockwise {
        Direction::Clockwise
    } else {
        Direction::Anticlockwise
    };
    TranslatedDragForWholeFace { face, direction }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use crate::gui::MouseControl;

    #[test]
    fn test_inner_camera_controls_target_the_same_point() {
        let target = Vec3 {
            x: 1.234,
            y: 4.321,
            z: 0.453,
        };

        let mouse_control = MouseControl::new(target, 0.1, 5.0);

        assert_eq!(target, mouse_control.free_orbit.target);
        assert_eq!(target, mouse_control.upright_orbit.target);
    }

    mod pick_to_face {
        use super::*;

        use pretty_assertions::assert_eq;

        #[test]
        fn front() {
            let pick_for_front = Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            };

            let face = pick_to_face(pick_for_front);

            assert_eq!(face, Some(Face::Front));
        }

        #[test]
        fn back() {
            let pick_for_back = Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            };

            let face = pick_to_face(pick_for_back);

            assert_eq!(face, Some(Face::Back));
        }

        #[test]
        fn left() {
            let pick_for_left = Vec3 {
                x: -1.,
                y: 0.,
                z: 0.,
            };

            let face = pick_to_face(pick_for_left);

            assert_eq!(face, Some(Face::Left));
        }

        #[test]
        fn right() {
            let pick_for_right = Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            };

            let face = pick_to_face(pick_for_right);

            assert_eq!(face, Some(Face::Right));
        }

        #[test]
        fn up() {
            let pick_for_up = Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            };

            let face = pick_to_face(pick_for_up);

            assert_eq!(face, Some(Face::Up));
        }

        #[test]
        fn down() {
            let pick_for_down = Vec3 {
                x: 0.,
                y: -1.,
                z: 0.,
            };

            let face = pick_to_face(pick_for_down);

            assert_eq!(face, Some(Face::Down));
        }

        #[test]
        fn invalid_does_not_panic() {
            let pick_for_down = Vec3 {
                x: 0.,
                y: 0.5,
                z: 0.,
            };

            let face = pick_to_face(pick_for_down);

            assert_eq!(face, None);
        }
    }

    mod validate_straight_dir {
        use super::*;

        #[test]
        fn too_small() {
            let unrotated_start_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: 1.01,
                y: 1.,
                z: 1.,
            };

            let validated_straight_dir_opt =
                validate_straight_dir(unrotated_start_pick, unrotated_end_pick);

            assert!(validated_straight_dir_opt.is_none());
        }

        #[test]
        fn diagonal() {
            let unrotated_start_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: -1.,
                y: -1.,
                z: 1.,
            };

            let validated_straight_dir_opt =
                validate_straight_dir(unrotated_start_pick, unrotated_end_pick);

            assert!(validated_straight_dir_opt.is_none());
        }

        #[test]
        fn valid_positive_x() {
            let unrotated_start_pick = Vec3 {
                x: -1.,
                y: 1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };

            let ValidatedStraightDir {
                move_along_x,
                toward_positive,
            } = validate_straight_dir(unrotated_start_pick, unrotated_end_pick).unwrap();

            assert!(move_along_x);
            assert!(toward_positive);
        }

        #[test]
        fn valid_negative_x() {
            let unrotated_start_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: -1.,
                y: 1.,
                z: 1.,
            };

            let ValidatedStraightDir {
                move_along_x,
                toward_positive,
            } = validate_straight_dir(unrotated_start_pick, unrotated_end_pick).unwrap();

            assert!(move_along_x);
            assert!(!toward_positive);
        }

        #[test]
        fn valid_positive_y() {
            let unrotated_start_pick = Vec3 {
                x: 1.,
                y: -1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };

            let ValidatedStraightDir {
                move_along_x,
                toward_positive,
            } = validate_straight_dir(unrotated_start_pick, unrotated_end_pick).unwrap();

            assert!(!move_along_x);
            assert!(toward_positive);
        }

        #[test]
        fn valid_negative_y() {
            let unrotated_start_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            let unrotated_end_pick = Vec3 {
                x: 1.,
                y: -1.,
                z: 1.,
            };

            let ValidatedStraightDir {
                move_along_x,
                toward_positive,
            } = validate_straight_dir(unrotated_start_pick, unrotated_end_pick).unwrap();

            assert!(!move_along_x);
            assert!(!toward_positive);
        }
    }

    mod picks_to_move {
        use super::*;

        use pretty_assertions::assert_eq;
        use rusty_puzzle_cube::cube::rotation::RotationKind;

        #[test]
        fn too_small() {
            let side_length = 3;
            let start_pick = Vec3 {
                x: 1.,
                y: 0.,
                z: 1.,
            };
            let end_pick = Vec3 {
                x: 1.01,
                y: 0.,
                z: 1.,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Front);

            assert!(decided_move.is_none());
        }

        #[test]
        fn diagonal() {
            let side_length = 3;
            let start_pick = Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            };
            let end_pick = Vec3 {
                x: -1.,
                y: -1.,
                z: 1.,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Front);

            assert!(decided_move.is_none());
        }

        #[test]
        fn dragged_front_along_the_top() {
            let side_length = 3;
            let start_pick = Vec3 {
                x: -0.8,
                y: 0.95,
                z: 1.,
            };
            let end_pick = Vec3 {
                x: 0.8,
                y: 0.95,
                z: 1.,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Front);

            assert!(matches!(
                decided_move,
                Some(DecidedMove::WholeFace {
                    face: Face::Up,
                    direction: Direction::Anticlockwise,
                })
            ));

            let rotation = decided_move.unwrap().as_rotation().normalise(side_length);

            assert_eq!(
                rotation,
                Rotation {
                    relative_to: Face::Up,
                    direction: Direction::Anticlockwise,
                    kind: RotationKind::FaceOnly,
                }
            );
        }

        #[test]
        fn dragged_right_backwards_along_the_middle() {
            let side_length = 3;
            let start_pick = Vec3 {
                x: 1.,
                y: 0.,
                z: -0.8,
            };
            let end_pick = Vec3 {
                x: 1.,
                y: 0.,
                z: 0.8,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Right);

            assert!(matches!(
                decided_move,
                Some(DecidedMove::InnerRow {
                    face: Face::Right,
                    row: 1,
                    toward_positive: false,
                })
            ));

            let rotation = decided_move.unwrap().as_rotation().normalise(side_length);

            assert_eq!(
                rotation,
                Rotation {
                    relative_to: Face::Down,
                    direction: Direction::Anticlockwise,
                    kind: RotationKind::Setback { layer: 1 },
                }
            );
        }

        #[test]
        fn dragged_up_downwards_along_the_middle() {
            let side_length = 4;
            let start_pick = Vec3 {
                x: 0.1,
                y: 1.,
                z: -0.8,
            };
            let end_pick = Vec3 {
                x: 0.1,
                y: 1.,
                z: 0.8,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Up);

            assert!(matches!(
                decided_move,
                Some(DecidedMove::InnerCol {
                    face: Face::Up,
                    col: 2,
                    toward_positive: false,
                })
            ));

            let rotation = decided_move.unwrap().as_rotation().normalise(side_length);

            assert_eq!(
                rotation,
                Rotation {
                    relative_to: Face::Left,
                    direction: Direction::Clockwise,
                    kind: RotationKind::Setback { layer: 2 },
                }
            );
        }

        #[test]
        fn dragged_left_upwards_along_the_inner_left() {
            let side_length = 5;
            let start_pick = Vec3 {
                x: -1.,
                y: -0.9,
                z: -0.5,
            };
            let end_pick = Vec3 {
                x: -1.,
                y: 0.9,
                z: -0.5,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Left);

            assert!(matches!(
                decided_move,
                Some(DecidedMove::InnerCol {
                    face: Face::Left,
                    col: 1,
                    toward_positive: true,
                })
            ));

            let rotation = decided_move.unwrap().as_rotation().normalise(side_length);

            assert_eq!(
                rotation,
                Rotation {
                    relative_to: Face::Back,
                    direction: Direction::Anticlockwise,
                    kind: RotationKind::Setback { layer: 1 },
                }
            );
        }

        #[test]
        fn dragged_left_downwards_along_the_left() {
            let side_length = 5;
            let start_pick = Vec3 {
                x: -1.,
                y: 0.9,
                z: -0.8,
            };
            let end_pick = Vec3 {
                x: -1.,
                y: -0.9,
                z: -0.8,
            };

            let decided_move = picks_to_move(side_length, start_pick, end_pick, Face::Left);

            assert!(matches!(
                decided_move,
                Some(DecidedMove::WholeFace {
                    face: Face::Back,
                    direction: Direction::Clockwise,
                })
            ));

            let rotation = decided_move.unwrap().as_rotation().normalise(side_length);

            assert_eq!(
                rotation,
                Rotation {
                    relative_to: Face::Back,
                    direction: Direction::Clockwise,
                    kind: RotationKind::FaceOnly,
                }
            );
        }
    }
}
