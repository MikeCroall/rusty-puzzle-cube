use std::ops::Not;

use rand::Rng;

use super::{direction::Direction, face::Face};

/// A struct representing the rotation of a 'slice' of cube.
///
/// Uses a specific face as an anchor point for the direction of the rotation, as well for which layers should be included.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rotation {
    /// The face from which the reference frame is anchored.
    pub relative_to: Face,

    /// Whether the rotation should be clockwise, using the reference frame of the face `relative_to`.
    pub direction: Direction,

    /// Specifies which layer(s) are included in this `Rotation`.
    pub kind: RotationKind,
}

/// Represents the layer(s) that are included in a given `Rotation`.
///
/// Some variants will include indices of layers which are all in the reference frame of `relative_to` from the outer `Rotation` struct.
/// Here are some examples of what different layer indices mean.
///
/// A value of `0` would be the face itself, which would typically not be used as `FaceOnly` represents this case more simply.
///
/// A value of `1` would be the layer immediately behind the face layer. This would be the middle layer on a 3x3x3 cube.
///
/// A value of `2` would be the layer behind layer `1`, further away from the `relative_to` face.
///
/// A value equal to `side length - 1` would be the opposite face to that specified by `relative_to`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RotationKind {
    /// Only the layer of the `relative_to` face will be affected.
    ///
    /// An example rotation of this kind is `R`.
    FaceOnly,

    /// All layers from the `relative_to` face to the `layer` index inclusive will be affected.
    /// This is sometimes called a wide rotation.
    ///
    /// An example rotation of this kind is `Rw` where `layer` would be `1`, or `3Rw` where `layer` would be `2`.
    Multilayer {
        /// How far 'in' to the cube the final layer to rotate is. Layer indices between `0` and `layer` inclusive will be included in this rotation.
        layer: usize,
    },

    /// One layer will be affected, determined by the `layer` index and the `relative_to` face.
    ///
    /// An example rotation of this kind is `3R` where `layer` would be `2`.
    Setback {
        /// How far 'in' to the cube the layer to rotate is. Only layer index `layer` will be included in this rotation.
        layer: usize,
    },
    //
    // todo: MultiSetback{pub start_layer: usize, pub end_layer: usize} for e.g. `3-5R` which would be indices start layer `2` end layer `4`
}

impl Rotation {
    /// Construct a `Rotation` that will turn `face` 90° clockwise from the perspective of looking directly at that face from outside the cube.
    #[must_use]
    pub fn clockwise(face: Face) -> Rotation {
        Rotation {
            relative_to: face,
            direction: Direction::Clockwise,
            kind: RotationKind::FaceOnly,
        }
    }

    /// Construct a `Rotation` that will turn `face` 90° anticlockwise from the perspective of looking directly at that face from outside the cube.
    #[must_use]
    pub fn anticlockwise(face: Face) -> Rotation {
        Rotation {
            relative_to: face,
            direction: Direction::Anticlockwise,
            kind: RotationKind::FaceOnly,
        }
    }

    /// Construct a `Rotation` that will turn a given layer of the cube 90° clockwise from the perspective of looking directly at `face` from outside the cube. The layer is chosen by providing an index where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn clockwise_setback_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            direction: Direction::Clockwise,
            kind: RotationKind::Setback { layer: layers_back },
        }
    }

    /// Construct a `Rotation` that will turn a given layer of the cube 90° anticlockwise from the perspective of looking directly at `face` from outside the cube. The layer is chosen by providing an index where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn anticlockwise_setback_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            direction: Direction::Anticlockwise,
            kind: RotationKind::Setback { layer: layers_back },
        }
    }

    /// Construct a `Rotation` that will turn multiple layers of the cube 90° clockwise from the perspective of looking directly at `face` from outside the cube. The layers start from the `face` layer and extend into the cube as far as `layers_back` where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn clockwise_multilayer_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            direction: Direction::Clockwise,
            kind: RotationKind::Multilayer { layer: layers_back },
        }
    }

    /// Construct a `Rotation` that will turn multiple layers of the cube 90° anticlockwise from the perspective of looking directly at `face` from outside the cube. The layers start from the `face` layer and extend into the cube as far as `layers_back` where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn anticlockwise_multilayer_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            direction: Direction::Anticlockwise,
            kind: RotationKind::Multilayer { layer: layers_back },
        }
    }

    /// Construct a randomly generated `Rotation`. The `Rotation` will be valid for a `Cube` of at least `side_length` cubies wide.
    /// This `Rotation` is expected to be used via `rotate` on a `Cube`, meaning it makes no attempt to avoid unusual edge cases such as picking the furthest layer away from `relative_to`.
    #[must_use]
    pub fn random(side_length: usize) -> Rotation {
        let mut rng = rand::rng();
        let relative_to: Face = rand::random();
        let layer = rng.random_range(0..side_length);
        let direction = if rng.random_bool(0.5) {
            Direction::Clockwise
        } else {
            Direction::Anticlockwise
        };
        let multilayer = rng.random_bool(0.333);

        let kind = if layer == 0 {
            RotationKind::FaceOnly
        } else if multilayer {
            RotationKind::Multilayer { layer }
        } else {
            RotationKind::Setback { layer }
        };

        Rotation {
            relative_to,
            direction,
            kind,
        }
    }

    /// Ensure that this `Rotation` does not have a `layer` that corresponds to the `Face` opposite to the one this `Rotation` is `relative_to`.
    /// That is, if this `Rotation` is `relative_to` the `Front` face, with a `layer` that means it actually turns the `Back` face, return a `Rotation` that is `relative_to` the `Back` face with a `layer` of 0.
    /// The `direction` is also flipped such that the semantics of the rotation are maintained.
    /// This applies to any pair of opposite faces.
    #[must_use]
    pub fn normalise(self, side_length: usize) -> Rotation {
        let furthest_layer = side_length - 1;

        if side_length > 1
            && matches!(self.kind, RotationKind::Setback { layer } if layer == furthest_layer)
        {
            self.as_layer_0_of_opposite_face()
        } else {
            self
        }
    }

    pub(crate) fn as_layer_0_of_opposite_face(self) -> Rotation {
        Rotation {
            relative_to: !self.relative_to,
            direction: !self.direction,
            kind: RotationKind::FaceOnly,
        }
    }
}

impl Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        Rotation {
            direction: !self.direction,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn clockwise() {
        let cw = Rotation::clockwise(Face::Back);
        let expected_output = Rotation {
            relative_to: Face::Back,
            direction: Direction::Clockwise,
            kind: RotationKind::FaceOnly,
        };
        assert_eq!(expected_output, cw);
    }

    #[test]
    fn anticlockwise() {
        let acw = Rotation::anticlockwise(Face::Right);
        let expected_output = Rotation {
            relative_to: Face::Right,
            direction: Direction::Anticlockwise,
            kind: RotationKind::FaceOnly,
        };
        assert_eq!(expected_output, acw);
    }

    #[test]
    fn clockwise_setback_from() {
        let cwsb = Rotation::clockwise_setback_from(Face::Down, 3);
        let expected_output = Rotation {
            relative_to: Face::Down,
            direction: Direction::Clockwise,
            kind: RotationKind::Setback { layer: 3 },
        };
        assert_eq!(expected_output, cwsb);
    }

    #[test]
    fn anticlockwise_setback_from() {
        let acwsb = Rotation::anticlockwise_setback_from(Face::Front, 4);
        let expected_output = Rotation {
            relative_to: Face::Front,
            direction: Direction::Anticlockwise,
            kind: RotationKind::Setback { layer: 4 },
        };
        assert_eq!(expected_output, acwsb);
    }

    #[test]
    fn clockwise_multilayer_from() {
        let cwsb = Rotation::clockwise_multilayer_from(Face::Down, 3);
        let expected_output = Rotation {
            relative_to: Face::Down,
            direction: Direction::Clockwise,
            kind: RotationKind::Multilayer { layer: 3 },
        };
        assert_eq!(expected_output, cwsb);
    }

    #[test]
    fn anticlockwise_multilayer_from() {
        let acwsb = Rotation::anticlockwise_multilayer_from(Face::Front, 4);
        let expected_output = Rotation {
            relative_to: Face::Front,
            direction: Direction::Anticlockwise,
            kind: RotationKind::Multilayer { layer: 4 },
        };
        assert_eq!(expected_output, acwsb);
    }

    #[test]
    fn normalise_already_normalised() {
        let input = Rotation {
            relative_to: Face::Up,
            direction: Direction::Clockwise,
            kind: RotationKind::Setback { layer: 7 },
        };
        let expected_output = input;
        assert_eq!(expected_output, input.normalise(9));
    }

    #[test]
    fn normalise_already_normalised_only_because_multilayer() {
        let input = Rotation {
            relative_to: Face::Up,
            direction: Direction::Clockwise,
            kind: RotationKind::Multilayer { layer: 7 },
        };
        let expected_output = input;
        assert_eq!(expected_output, input.normalise(8));
    }

    #[test]
    fn normalise_not_already_normalised() {
        let input = Rotation {
            relative_to: Face::Up,
            direction: Direction::Clockwise,
            kind: RotationKind::Setback { layer: 7 },
        };
        let expected_output = Rotation {
            relative_to: Face::Down,
            direction: Direction::Anticlockwise,
            kind: RotationKind::FaceOnly,
        };
        assert_eq!(expected_output, input.normalise(8));
    }

    #[test]
    fn as_layer_0_of_opposite_face() {
        let input = Rotation {
            relative_to: Face::Up,
            direction: Direction::Clockwise,
            kind: RotationKind::Setback { layer: 7 },
        };
        let expected_output = Rotation {
            relative_to: Face::Down,
            direction: Direction::Anticlockwise,
            kind: RotationKind::FaceOnly,
        };
        assert_eq!(expected_output, input.as_layer_0_of_opposite_face());
    }

    #[test]
    fn as_layer_0_of_opposite_face_multilayer() {
        let input = Rotation {
            relative_to: Face::Up,
            direction: Direction::Clockwise,
            kind: RotationKind::Multilayer { layer: 7 },
        };
        let expected_output = Rotation {
            relative_to: Face::Down,
            direction: Direction::Anticlockwise,
            kind: RotationKind::FaceOnly,
        };
        assert_eq!(expected_output, input.as_layer_0_of_opposite_face());
    }

    #[test]
    fn invert_only_changes_direction() {
        let relative_to = Face::Left;
        let layer = 4;
        let input = Rotation {
            relative_to,
            direction: Direction::Anticlockwise,
            kind: RotationKind::Multilayer { layer },
        };
        let expected_output = Rotation {
            relative_to,
            direction: Direction::Clockwise,
            kind: RotationKind::Multilayer { layer },
        };
        assert_eq!(expected_output, !input);
    }

    #[test]
    fn random_picks_layer_within_bounds() {
        let side_length = 5;

        for _ in 0..25 {
            let rotation = Rotation::random(side_length);

            assert!(
                matches!(rotation.kind, RotationKind::Setback { layer } | RotationKind::Multilayer { layer } if layer < side_length)
                    || matches!(rotation.kind, RotationKind::FaceOnly)
            );
        }
    }
}
