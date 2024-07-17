use anyhow::Context;

use super::{
    cubie_face::CubieFace, face::Face, face::IndexAlignment,
    helpers::get_clockwise_slice_of_side_setback, Cube,
};

/// A struct representing the rotation of a 'slice' of cube.
/// That is, a rotation of a set of cubies where none of the cubies lie on the edges of the cube.
#[derive(Copy, Clone)]
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
    pub fn rotate_inner_slice(
        &mut self,
        CubeSliceTwist {
            relative_to,
            layer,
            clockwise,
        }: CubeSliceTwist,
    ) -> anyhow::Result<()> {
        self.rotate_setback_adjacents_clockwise(relative_to, layer)?;

        if !clockwise {
            self.rotate_setback_adjacents_clockwise(relative_to, layer)?;
            self.rotate_setback_adjacents_clockwise(relative_to, layer)?;
        }

        Ok(())
    }

    fn rotate_setback_adjacents_clockwise(
        &mut self,
        face: Face,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let adjacents = face.adjacent_faces_clockwise();
        let slice_0 = get_clockwise_slice_of_side_setback(
            &self.side_map[adjacents[0].0],
            &adjacents[0].1,
            layers_back,
        )?;
        let slice_1 = get_clockwise_slice_of_side_setback(
            &self.side_map[adjacents[1].0],
            &adjacents[1].1,
            layers_back,
        )?;
        let slice_2 = get_clockwise_slice_of_side_setback(
            &self.side_map[adjacents[2].0],
            &adjacents[2].1,
            layers_back,
        )?;
        let slice_3 = get_clockwise_slice_of_side_setback(
            &self.side_map[adjacents[3].0],
            &adjacents[3].1,
            layers_back,
        )?;

        let final_order = {
            let mut preliminary_order = adjacents.iter();
            let first_element = preliminary_order.next();
            preliminary_order
                .chain(first_element)
                .collect::<Vec<&(Face, IndexAlignment)>>()
        };

        self.copy_setback_adjacent_over(final_order[0], slice_0, layers_back)?;
        self.copy_setback_adjacent_over(final_order[1], slice_1, layers_back)?;
        self.copy_setback_adjacent_over(final_order[2], slice_2, layers_back)?;
        self.copy_setback_adjacent_over(final_order[3], slice_3, layers_back)?;

        Ok(())
    }

    fn copy_setback_adjacent_over(
        &mut self,
        (target_face, target_alignment): &(Face, IndexAlignment),
        unadjusted_values: Vec<CubieFace>,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let values = if target_alignment == &IndexAlignment::InnerFirst
            || target_alignment == &IndexAlignment::OuterEnd
        {
            let mut new_values = unadjusted_values.clone();
            new_values.reverse();
            new_values
        } else {
            unadjusted_values
        };

        let side = &mut self.side_map[*target_face];
        match target_alignment {
            IndexAlignment::OuterStart | IndexAlignment::OuterEnd => {
                let inner_index = match target_alignment {
                    IndexAlignment::OuterStart => layers_back - 1,
                    IndexAlignment::OuterEnd => self.side_length - layers_back,
                    _ => unreachable!("outer match guard clauses this one to only allow IndexAlignment::OuterStart and IndexAlignment::OuterEnd"),
                };
                for (i, value) in values.iter().enumerate() {
                    value.clone_into(&mut side[i][inner_index]);
                }
            }
            IndexAlignment::InnerFirst => {
                side.get_mut(layers_back - 1)
                    .with_context(|| "Side did not have requested layer")?
                    .clone_from_slice(&values);
            }
            IndexAlignment::InnerLast => {
                side.get_mut(self.side_length - layers_back)
                    .with_context(|| "Side did not have requested layer")?
                    .clone_from_slice(&values);
            }
        };

        Ok(())
    }
}
