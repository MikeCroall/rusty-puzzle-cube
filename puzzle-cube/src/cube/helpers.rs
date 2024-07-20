use std::vec;

use anyhow::Context;

use super::{
    cubie_face::CubieFace,
    face::IndexAlignment as IA,
    side_lengths::{SideLength, UniqueCharsSideLength},
    Side,
};

pub(super) fn create_side(
    side_length: SideLength,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    let side_length = side_length.into();
    let mut side = vec![];
    for _outer in 0..side_length {
        let inner_vec = vec![colour_variant_creator(None); side_length];
        side.push(inner_vec);
    }
    side
}

pub(super) fn create_side_with_unique_characters(
    side_length: UniqueCharsSideLength,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    let side_length = side_length.into();
    let mut side = vec![];
    for outer in 0..side_length {
        let mut inner_vec = vec![];
        for inner in 0..side_length {
            let value = u32::try_from((side_length * outer) + inner)
                .expect("side_length is limited to 8 so conversion to u32 should never fail");
            let display_char = char::from_u32('0' as u32 + value);
            inner_vec.push(colour_variant_creator(display_char));
        }
        side.push(inner_vec);
    }
    side
}

pub(super) fn get_clockwise_slice_of_side_setback(
    side: &Side,
    index_alignment: &IA,
    layers_back: usize,
) -> anyhow::Result<Vec<CubieFace>> {
    Ok(match index_alignment {
        IA::OuterStart => side
            .iter()
            .map(|inner| -> anyhow::Result<CubieFace> {
                Ok(inner
                    .get(layers_back)
                    .with_context(|| format!("Side did not have requested layer ({layers_back} of inner vec of side)"))?
                    .to_owned())
            })
            .collect::<anyhow::Result<Vec<CubieFace>>>()?,
        IA::OuterEnd => side
            .iter()
            .map(|inner| -> anyhow::Result<CubieFace> {
                Ok(inner
                    .get(inner.len() - layers_back - 1)
                    .with_context(|| format!("Side did not have requested layer ({} of inner vec of side)", inner.len() - layers_back - 1))?
                    .to_owned())
            })
            .rev()
            .collect::<anyhow::Result<Vec<CubieFace>>>()?,
        IA::InnerFirst => {
            let mut inner_first_vec = side
                .get(layers_back)
                .with_context(|| format!("Side did not have requested layer ({layers_back} of outer vec of side)"))?
                .to_owned();
            inner_first_vec.reverse();
            inner_first_vec
        }
        IA::InnerLast => side
            .get(side.len() - layers_back - 1)
            .with_context(|| format!("Side did not have requested layer ({} of outer vec of side)", side.len() - layers_back - 1))?
            .to_owned(),
    })
}
