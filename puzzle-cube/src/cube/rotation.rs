use std::ops::Not;

use rand::Rng;

use super::{direction::Direction, face::Face};

/// A struct representing the rotation of a 'slice' of cube.
/// That is, a rotation of a set of cubies where none of the cubies lie on the edges of the cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rotation {
    /// The face from which the reference frame is anchored.
    /// `layer` will determine how many layers 'behind' this face the desired slice to rotate is.
    pub relative_to: Face,

    /// How far 'in' to the cube the layer to rotate is.
    /// A value of 0 would be the face itself, which would not technically be a slice twist, but a whole face twist. This is a special case.
    /// A value of 1 would be the layer immediately behind the face layer.
    /// A value of 2 would be the layer behind layer 1, further away from the `relative_to` face.
    /// A value equal to side length - 1 would be the opposite face, which is also a special case.
    pub layer: usize,

    /// Whether the rotation should be clockwise, using the reference frame of the face `relative_to`.
    pub direction: Direction,
}

impl Rotation {
    /// Construct a `Rotation` that will turn `face` 90° clockwise from the perspective of looking directly at that face from outside the cube.
    #[must_use]
    pub fn clockwise(face: Face) -> Rotation {
        Rotation {
            relative_to: face,
            layer: 0,
            direction: Direction::Clockwise,
        }
    }

    /// Construct a `Rotation` that will turn `face` 90° anticlockwise from the perspective of looking directly at that face from outside the cube.
    #[must_use]
    pub fn anticlockwise(face: Face) -> Rotation {
        Rotation {
            relative_to: face,
            layer: 0,
            direction: Direction::Anticlockwise,
        }
    }

    /// Construct a `Rotation` that will turn a given layer of the cube 90° clockwise from the perspective of looking directly at `face` from outside the cube. The layer is chosen by providing an index where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn clockwise_setback_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            layer: layers_back,
            direction: Direction::Clockwise,
        }
    }

    /// Construct a `Rotation` that will turn a given layer of the cube 90° anticlockwise from the perspective of looking directly at `face` from outside the cube. The layer is chosen by providing an index where `face` itself is 0, the layer immediately behind it is 1, and so on.
    #[must_use]
    pub fn anticlockwise_setback_from(relative_to: Face, layers_back: usize) -> Rotation {
        Rotation {
            relative_to,
            layer: layers_back,
            direction: Direction::Anticlockwise,
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
        Rotation {
            relative_to,
            layer,
            direction,
        }
    }

    /// Ensure that this `Rotation` does not have a `layer` that corresponds to the `Face` opposite to the one this `Rotation` is `relative_to`.
    /// That is, if this `Rotation` is `relative_to` the `Front` face, with a `layer` that means it actually turns the `Back` face, return a `Rotation` that is `relative_to` the `Back` face with a `layer` of 0.
    /// The `direction` is also flipped such that the semantics of the rotation are maintained.
    /// This applies to any pair of opposite faces.
    #[must_use]
    pub fn normalise(self, side_length: usize) -> Rotation {
        let furthest_layer = side_length - 1;
        if self.layer == furthest_layer && side_length > 1 {
            self.as_layer_0_of_opposite_face()
        } else {
            self
        }
    }

    pub(crate) fn as_layer_0_of_opposite_face(self) -> Rotation {
        Rotation {
            relative_to: !self.relative_to,
            layer: 0,
            direction: !self.direction,
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
            layer: 0,
            direction: Direction::Clockwise,
        };
        assert_eq!(expected_output, cw);
    }

    #[test]
    fn anticlockwise() {
        let acw = Rotation::anticlockwise(Face::Right);
        let expected_output = Rotation {
            relative_to: Face::Right,
            layer: 0,
            direction: Direction::Anticlockwise,
        };
        assert_eq!(expected_output, acw);
    }

    #[test]
    fn clockwise_setback_from() {
        let cwsb = Rotation::clockwise_setback_from(Face::Down, 3);
        let expected_output = Rotation {
            relative_to: Face::Down,
            layer: 3,
            direction: Direction::Clockwise,
        };
        assert_eq!(expected_output, cwsb);
    }

    #[test]
    fn anticlockwise_setback_from() {
        let acwsb = Rotation::anticlockwise_setback_from(Face::Front, 4);
        let expected_output = Rotation {
            relative_to: Face::Front,
            layer: 4,
            direction: Direction::Anticlockwise,
        };
        assert_eq!(expected_output, acwsb);
    }

    #[test]
    fn normalise_already_normalised() {
        let input = Rotation {
            relative_to: Face::Up,
            layer: 7,
            direction: Direction::Clockwise,
        };
        let expected_output = input;
        assert_eq!(expected_output, input.normalise(9));
    }

    #[test]
    fn normalise_not_already_normalised() {
        let input = Rotation {
            relative_to: Face::Up,
            layer: 7,
            direction: Direction::Clockwise,
        };
        let expected_output = Rotation {
            relative_to: Face::Down,
            layer: 0,
            direction: Direction::Anticlockwise,
        };
        assert_eq!(expected_output, input.normalise(8));
    }

    #[test]
    fn as_layer_0_of_opposite_face() {
        let input = Rotation {
            relative_to: Face::Up,
            layer: 7,
            direction: Direction::Clockwise,
        };
        let expected_output = Rotation {
            relative_to: Face::Down,
            layer: 0,
            direction: Direction::Anticlockwise,
        };
        assert_eq!(expected_output, input.as_layer_0_of_opposite_face());
    }

    #[test]
    fn invert_only_changes_direction() {
        let relative_to = Face::Left;
        let layer = 4;
        let input = Rotation {
            relative_to,
            layer,
            direction: Direction::Anticlockwise,
        };
        let expected_output = Rotation {
            relative_to,
            layer,
            direction: Direction::Clockwise,
        };
        assert_eq!(expected_output, !input);
    }

    #[test]
    fn random_picks_layer_within_bounds() {
        let side_length = 5;

        for _ in 0..25 {
            let rotation = Rotation::random(side_length);
            assert!(rotation.layer < side_length);
        }
    }
}
