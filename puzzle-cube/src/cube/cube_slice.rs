use super::{face::Face, Cube};

/// A struct representing the rotation of a 'slice' of cube.
/// That is, a rotation of a set of cubies where none of the cubies lie on the edges of the cube.
pub struct CubeSliceTwist {
    /// The face from which the reference frame is anchored.
    /// `layer` will determine how many layers 'behind' this face the desired slice to rotate is.
    pub relative_to: Face,

    /// How far 'in' to the cube the layer to rotate is.
    /// A value of 0 would be the face itself, which would not technically be a slice twist, but a whole face twist. This is a special case. // todo do we disallow face twist when using this struct?
    /// A value of 1 would be the layer immediately behind the face layer.
    pub layer: usize,

    /// Whether the rotation should be clockwise, using the reference frame of the face `relative_to`.
    pub clockwise: bool,
}

impl Cube {
    pub fn twist_inner_slice(
        &mut self,
        CubeSliceTwist {
            relative_to,
            layer,
            clockwise,
        }: CubeSliceTwist,
    ) -> anyhow::Result<(), String> {
        todo!("impl twist_inner_slice (and refactor rotation of 'adjacents' to use it too?)");
    }
}
