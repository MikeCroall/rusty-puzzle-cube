use super::{cubie_face::CubieFace, Side};
use crate::cube::IA;

pub(super) fn create_side(
    side_length: usize,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    assert!(side_length > 0, "create_side must have 1 <= side_length");
    let mut side = vec![];
    for _outer in 0..side_length {
        let inner_vec = vec![colour_variant_creator(None); side_length];
        side.push(inner_vec);
    }
    side
}

pub(super) fn create_side_with_unique_characters(
    side_length: usize,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    assert!(
        (1..=8).contains(&side_length),
        "create_side_with_unique_characters must have 1 <= side_length <= 8"
    );
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

pub(super) fn get_clockwise_slice_of_side(side: &Side, index_alignment: &IA) -> Vec<CubieFace> {
    match index_alignment {
        IA::OuterStart => side
            .iter()
            .map(|inner| inner.first().expect("Side inner had no member").to_owned())
            .collect::<Vec<CubieFace>>(),
        IA::OuterEnd => side
            .iter()
            .map(|inner| inner.last().expect("Side inner had no member").to_owned())
            .rev()
            .collect::<Vec<CubieFace>>(),
        IA::InnerFirst => {
            let mut inner_first_vec = side.first().expect("Side had no inner").to_owned();
            inner_first_vec.reverse();
            inner_first_vec
        }
        IA::InnerLast => side.last().expect("Side had no inner").to_owned(),
    }
}
