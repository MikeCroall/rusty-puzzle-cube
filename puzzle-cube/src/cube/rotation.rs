use super::{direction::Direction, face::Face};

/// A struct representing the rotation of a 'slice' of cube.
/// That is, a rotation of a set of cubies where none of the cubies lie on the edges of the cube.
#[derive(Copy, Clone)]
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

    pub(crate) fn reverse_direction(self) -> Rotation {
        Rotation {
            direction: !self.direction,
            ..self
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
