use rusty_puzzle_cube::cube::{direction::Direction, face::Face, rotation::Rotation};

pub(super) enum DecidedMove {
    // todo can/should we remove DecidedMove and go straight to Rotation?
    WholeFace {
        face: Face,
        direction: Direction,
    },
    InnerRow {
        face: Face,
        row: usize,
        toward_positive: bool,
    },
    InnerCol {
        face: Face,
        col: usize,
        toward_positive: bool,
    },
}

impl DecidedMove {
    pub(super) fn as_rotation(&self) -> Rotation {
        match *self {
            DecidedMove::WholeFace { face, direction } => match direction {
                Direction::Clockwise => Rotation::clockwise(face),
                Direction::Anticlockwise => Rotation::anticlockwise(face),
            },
            DecidedMove::InnerRow {
                face,
                row,
                toward_positive: clockwise,
            } => {
                let face_where_toward_positive_eq_clockwise = match face {
                    Face::Up => Face::Front,
                    Face::Down => Face::Back,
                    Face::Front | Face::Right | Face::Back | Face::Left => Face::Down,
                };
                if clockwise {
                    Rotation::clockwise_setback_from(face_where_toward_positive_eq_clockwise, row)
                } else {
                    Rotation::anticlockwise_setback_from(
                        face_where_toward_positive_eq_clockwise,
                        row,
                    )
                }
            }
            DecidedMove::InnerCol {
                face,
                col,
                toward_positive: anticlockwise,
            } => {
                let face_where_toward_positive_eq_anticlockwise = match face {
                    Face::Up | Face::Down | Face::Front => Face::Left,
                    Face::Right => Face::Front,
                    Face::Back => Face::Right,
                    Face::Left => Face::Back,
                };
                if anticlockwise {
                    Rotation::anticlockwise_setback_from(
                        face_where_toward_positive_eq_anticlockwise,
                        col,
                    )
                } else {
                    Rotation::clockwise_setback_from(
                        face_where_toward_positive_eq_anticlockwise,
                        col,
                    )
                }
            }
        }
    }
}
