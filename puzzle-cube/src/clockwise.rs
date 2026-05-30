use std::mem;

use anyhow::Context;

use super::cube::{
    Cube, DefaultSide, PuzzleCube as _,
    cubie_face::CubieFace,
    face::{Face as F, IndexAlignment as IA},
};

pub(super) fn get_clockwise_slice_of_side_setback(
    side: &DefaultSide,
    index_alignment: IA,
    layers_back: usize,
) -> anyhow::Result<Vec<CubieFace>> {
    let vec = match index_alignment {
        IA::OuterStart => side
            .iter()
            .map(|inner| -> anyhow::Result<CubieFace> {
                Ok(inner
                    .get(layers_back)
                    .with_context(|| format!("side did not have required layer ({layers_back} of inner vec of side)"))?
                    .to_owned())
            })
            .collect::<anyhow::Result<Vec<CubieFace>>>()?,
        IA::OuterEnd => side
            .iter()
            .map(|inner| -> anyhow::Result<CubieFace> {
                let required_index = inner.len().checked_sub(layers_back + 1)
                    .with_context(|| format!("requested layer index {layers_back} caused underflow"))?;
                Ok(inner
                    .get(required_index)
                    .with_context(|| format!("side did not have required layer ({required_index} of inner vec of side)"))?
                    .to_owned())
            })
            .rev()
            .collect::<anyhow::Result<Vec<CubieFace>>>()?,
        IA::InnerFirst => {
            let mut inner_first_vec = side
                .get(layers_back)
                .with_context(|| format!("side did not have required layer ({layers_back} of outer vec of side)"))?
                .to_owned();
            inner_first_vec.reverse();
            inner_first_vec
        }
        IA::InnerLast => {
            let required_index = side.len().checked_sub(layers_back + 1)
                .with_context(|| format!("requested layer index {layers_back} caused underflow"))?;
            side
                .get(required_index)
                .with_context(|| format!("side did not have required layer ({required_index} of outer vec of side)"))?
                .to_owned()
        }
    };
    Ok(vec)
}

impl Cube {
    pub(super) fn rotate_layer_clockwise(
        &mut self,
        face: F,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        if layers_back == 0 {
            self.rotate_face_90_degrees_without_adjacents_clockwise(face);
        } else if layers_back == self.side_length - 1 {
            self.rotate_face_90_degrees_without_adjacents_clockwise(!face);
        }
        self.rotate_adjacents_90_deg_setback_clockwise(face, layers_back)
    }

    fn rotate_face_90_degrees_without_adjacents_clockwise(&mut self, face: F) {
        let side_length = self.side_length;
        let side: &mut Vec<Vec<CubieFace>> = self.side_mut(face);
        side.reverse();
        for i in 1..side_length {
            let (left, right) = side.split_at_mut(i);
            (0..i).for_each(|j| {
                mem::swap(&mut left[j][i], &mut right[0][j]);
            });
        }
    }

    fn rotate_adjacents_90_deg_setback_clockwise(
        &mut self,
        face: F,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let adjacents = face.adjacent_faces_clockwise();

        let slices = adjacents
            .map(|adj| get_clockwise_slice_of_side_setback(self.side(adj.0), adj.1, layers_back));

        adjacents
            .iter()
            .cycle()
            .skip(1)
            .zip(slices)
            .try_for_each(|(adjacent, slice)| {
                self.copy_setback_adjacent_over_clockwise(*adjacent, slice?, layers_back)
            })
    }

    fn copy_setback_adjacent_over_clockwise(
        &mut self,
        (target_face, target_alignment): (F, IA),
        unadjusted_values: Vec<CubieFace>,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let values = match target_alignment {
            IA::OuterEnd | IA::InnerFirst => {
                let mut new_values = unadjusted_values;
                new_values.reverse();
                new_values
            }
            IA::OuterStart | IA::InnerLast => unadjusted_values,
        };

        let side_length = self.side_length;
        let side = self.side_mut(target_face);
        match target_alignment {
            IA::OuterStart | IA::OuterEnd => {
                let inner_index = if target_alignment == IA::OuterStart {
                    layers_back
                } else {
                    side_length.checked_sub(layers_back + 1).with_context(|| {
                        format!("requested layer index {layers_back} caused underflow")
                    })?
                };
                for (outer_index, value) in values.iter().enumerate() {
                    side.get_mut(outer_index)
                        .with_context(|| format!("side did not have requested layer ({outer_index} of outer vec of side)"))?
                        .get_mut(inner_index)
                        .with_context(|| format!("side did not have requested layer ({inner_index} of inner vec of side)"))?
                        .clone_from(value);
                }
            }
            IA::InnerFirst | IA::InnerLast => {
                let outer_index = if target_alignment == IA::InnerFirst {
                    layers_back
                } else {
                    side_length.checked_sub(layers_back + 1).with_context(|| {
                        format!("requested layer index {layers_back} caused underflow")
                    })?
                };
                side.get_mut(outer_index)
                    .with_context(|| {
                        format!(
                            "side did not have requested layer ({outer_index} outer vec of side)"
                        )
                    })?
                    .clone_from_slice(&values);
            }
        }
        Ok(())
    }
}
