use std::vec;

use anyhow::Context;

use super::{
    DefaultSide,
    cubie_face::CubieFace,
    face::IndexAlignment as IA,
    flat_side::FlatSide,
    side_lengths::{SideLength, UniqueCharsSideLength},
};

pub(super) fn create_side(
    side_length: SideLength,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> DefaultSide {
    let side_length = side_length.into();
    let vec = vec![colour_variant_creator(None); side_length * side_length];
    FlatSide::new(side_length, vec)
}

pub(super) fn create_side_with_unique_characters(
    side_length: UniqueCharsSideLength,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> DefaultSide {
    let side_length = side_length.into();
    let mut side = Vec::with_capacity(side_length * side_length);
    for row in 0..side_length {
        for column in 0..side_length {
            let value = u32::try_from((side_length * row) + column)
                .expect("side_length is limited to 8 so conversion to u32 should never fail");
            let display_char = char::from_u32('0' as u32 + value);
            side.push(colour_variant_creator(display_char));
        }
    }
    FlatSide::new(side_length, side)
}

pub(super) fn get_clockwise_slice_of_side_setback(
    side: &DefaultSide,
    index_alignment: IA,
    layers_back: usize,
) -> anyhow::Result<Vec<CubieFace>> {
    Ok(match index_alignment {
        IA::OuterStart => side
            .col(layers_back)
            .with_context(|| format!("side did not have required column ({layers_back})"))?
            .copied()
            .collect(),
        IA::OuterEnd => {
            let required_index = side
                .side_length()
                .checked_sub(layers_back + 1)
                .with_context(|| format!("requested layer index {layers_back} caused underflow"))?;
            side.col(required_index)
                .with_context(|| format!("side did not have required column ({layers_back})"))?
                .rev()
                .copied()
                .collect()
        }
        IA::InnerFirst => {
            let mut inner_first_vec = side
                .row(layers_back)
                .with_context(|| format!("side did not have required row ({layers_back})"))?
                .to_owned();
            inner_first_vec.reverse();
            inner_first_vec
        }
        IA::InnerLast => {
            let required_index = side
                .side_length()
                .checked_sub(layers_back + 1)
                .with_context(|| format!("requested layer index {layers_back} caused underflow"))?;
            side.row(required_index)
                .with_context(|| format!("side did not have required row ({required_index})"))?
                .to_owned()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn only_use_visible_unique_characters() {
        let side = create_side_with_unique_characters(UniqueCharsSideLength::MAX, &CubieFace::Blue);

        side.iter_flat()
            .map(|cubie| cubie.get_coloured_display_char().input)
            .for_each(|string| {
                assert_eq!(
                    1,
                    string.chars().count(),
                    "Found too many chars: {string} (len: {})",
                    string.chars().count()
                );

                let char = string.chars().next().unwrap();
                assert!(
                    !char.is_whitespace(),
                    "Found whitespace char: {}",
                    char.escape_unicode()
                );
                assert!(
                    !char.is_control(),
                    "Found control char: {}",
                    char.escape_unicode()
                );
            });
    }
}
