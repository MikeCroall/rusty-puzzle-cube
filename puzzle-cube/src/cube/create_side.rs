use std::vec;

use super::{
    DefaultSide,
    cubie_face::CubieFace,
    side_lengths::{SideLength, UniqueCharsSideLength},
};

pub(super) fn create_side(
    side_length: SideLength,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> DefaultSide {
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
) -> DefaultSide {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn only_use_visible_unique_characters() {
        let side = create_side_with_unique_characters(UniqueCharsSideLength::MAX, &CubieFace::Blue);

        side.iter()
            .flatten()
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
