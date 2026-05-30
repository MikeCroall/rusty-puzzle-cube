use super::cube::{Cube, face::Face as F};

impl Cube {
    pub(super) fn rotate_layer_anticlockwise(
        &mut self,
        face: F,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        self.rotate_layer_clockwise(face, layers_back)?;
        self.rotate_layer_clockwise(face, layers_back)?;
        self.rotate_layer_clockwise(face, layers_back)?;
        Ok(())
    }
}
