use std::fmt;

use enum_map::{enum_map, EnumMap};
use itertools::izip;

use self::cubie_face::CubieFace;
use self::face::{Face as F, IndexAlignment as IA};

pub(crate) mod cubie_face;
pub(crate) mod face;
pub(crate) mod macros;

type Side = Vec<Vec<CubieFace>>;

const HORIZONTAL_PADDING: &str = " ";

#[derive(PartialEq)]
pub(crate) struct Cube {
    side_map: EnumMap<F, Box<Side>>,
}

impl Cube {
    pub(crate) fn create(side_length: usize) -> Self {
        Self {
            side_map: enum_map! {
                F::Top => Box::new(create_side(side_length, &CubieFace::White)),
                F::Bottom => Box::new(create_side(side_length, &CubieFace::Yellow)),
                F::Front => Box::new(create_side(side_length, &CubieFace::Blue)),
                F::Right => Box::new(create_side(side_length, &CubieFace::Orange)),
                F::Back => Box::new(create_side(side_length, &CubieFace::Green)),
                F::Left => Box::new(create_side(side_length, &CubieFace::Red)),
            },
        }
    }

    pub(crate) fn create_with_unique_characters(side_length: usize) -> Self {
        Self {
            side_map: enum_map! {
                F::Top => Box::new(create_side_with_unique_characters(side_length, &CubieFace::White)),
                F::Bottom => Box::new(create_side_with_unique_characters(side_length, &CubieFace::Yellow)),
                F::Front => Box::new(create_side_with_unique_characters(side_length, &CubieFace::Blue)),
                F::Right => Box::new(create_side_with_unique_characters(side_length, &CubieFace::Orange)),
                F::Back => Box::new(create_side_with_unique_characters(side_length, &CubieFace::Green)),
                F::Left => Box::new(create_side_with_unique_characters(side_length, &CubieFace::Red)),
            },
        }
    }

    pub(crate) fn rotate_face_90_degrees_clockwise(&mut self, face: F) {
        self.rotate_face_90_degrees_clockwise_without_adjacents(face);
        self.rotate_face_90_degrees_clockwise_only_adjacents(face);
    }

    pub(crate) fn rotate_face_90_degrees_anticlockwise(&mut self, face: F) {
        self.rotate_face_90_degrees_clockwise(face);
        self.rotate_face_90_degrees_clockwise(face);
        self.rotate_face_90_degrees_clockwise(face);
    }

    fn rotate_face_90_degrees_clockwise_without_adjacents(&mut self, face: F) {
        let side: &mut Vec<Vec<CubieFace>> = &mut self.side_map[face];
        side.reverse();
        for i in 1..side.len() {
            let (left, right) = side.split_at_mut(i);
            (0..i).for_each(|j| {
                std::mem::swap(&mut left[j][i], &mut right[0][j]);
            });
        }
    }

    fn rotate_face_90_degrees_clockwise_only_adjacents(&mut self, face: F) {
        let adjacents = face.adjacent_faces_clockwise();
        let slice_0 = get_clockwise_slice_of_side(&self.side_map[adjacents[0].0], &adjacents[0].1);
        let slice_1 = get_clockwise_slice_of_side(&self.side_map[adjacents[1].0], &adjacents[1].1);
        let slice_2 = get_clockwise_slice_of_side(&self.side_map[adjacents[2].0], &adjacents[2].1);
        let slice_3 = get_clockwise_slice_of_side(&self.side_map[adjacents[3].0], &adjacents[3].1);

        let final_order = {
            let mut preliminary_order = adjacents.iter();
            let first_element = preliminary_order.next();
            preliminary_order
                .chain(first_element)
                .collect::<Vec<&(F, IA)>>()
        };

        self.copy_adjacent_over(final_order[0], slice_0);
        self.copy_adjacent_over(final_order[1], slice_1);
        self.copy_adjacent_over(final_order[2], slice_2);
        self.copy_adjacent_over(final_order[3], slice_3);
    }

    fn copy_adjacent_over(
        &mut self,
        (target_face, target_alignment): &(F, IA),
        unadjusted_values: Vec<CubieFace>,
    ) {
        let values = if target_alignment == &IA::InnerFirst || target_alignment == &IA::OuterEnd {
            let mut new_values = unadjusted_values.clone();
            new_values.reverse();
            new_values
        } else {
            unadjusted_values
        };

        let side = &mut self.side_map[*target_face];
        match target_alignment {
            IA::OuterStart => {
                for i in 0..side.len() {
                    side[i][0] = values
                        .get(i)
                        .expect("Values had no element for index")
                        .to_owned();
                }
            }
            IA::OuterEnd => {
                let len = side.len();
                for i in 0..len {
                    side[i][len - 1] = values
                        .get(i)
                        .expect("Values had no element for index")
                        .to_owned();
                }
            }
            IA::InnerFirst => {
                side.first_mut()
                    .expect("Side had no inner")
                    .clone_from_slice(&values);
            }
            IA::InnerLast => {
                side.last_mut()
                    .expect("Side had no inner")
                    .clone_from_slice(&values);
            }
        }
    }

    fn write_indented_single_side(&self, f: &mut fmt::Formatter, face: F) -> fmt::Result {
        let side = &*self.side_map[face];
        let side_length = side.len();
        for cubie_row in side {
            write!(
                f,
                "{}",
                format!(" {HORIZONTAL_PADDING}").repeat(side_length)
            )?;
            Cube::write_cubie_row(f, cubie_row)?;
            writeln!(f)?;
        }
        Ok(())
    }

    fn write_unindented_four_sides(
        &self,
        f: &mut fmt::Formatter,
        face_a: F,
        face_b: F,
        face_c: F,
        face_d: F,
    ) -> fmt::Result {
        let side_a = self.side_map[face_a].iter();
        let side_b = self.side_map[face_b].iter();
        let side_c = self.side_map[face_c].iter();
        let side_d = self.side_map[face_d].iter();

        for (cubie_row_a, cubie_row_b, cubie_row_c, cubie_row_d) in
            izip!(side_a, side_b, side_c, side_d)
        {
            Cube::write_cubie_row(f, cubie_row_a)?;
            write!(f, "{HORIZONTAL_PADDING}")?;
            Cube::write_cubie_row(f, cubie_row_b)?;
            write!(f, "{HORIZONTAL_PADDING}")?;
            Cube::write_cubie_row(f, cubie_row_c)?;
            write!(f, "{HORIZONTAL_PADDING}")?;
            Cube::write_cubie_row(f, cubie_row_d)?;
            writeln!(f)?;
        }
        Ok(())
    }

    fn write_cubie_row(f: &mut fmt::Formatter, cubie_row: &[CubieFace]) -> fmt::Result {
        let joined_by_padding = cubie_row
            .iter()
            .map(|c| c.get_coloured_display_char().to_string())
            .collect::<Vec<String>>()
            .join(HORIZONTAL_PADDING);
        write!(f, "{joined_by_padding}")?;
        Ok(())
    }
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented_single_side(f, F::Top)?;
        self.write_unindented_four_sides(f, F::Left, F::Front, F::Right, F::Back)?;
        self.write_indented_single_side(f, F::Bottom)?;
        Ok(())
    }
}

#[cfg(test)]
impl Cube {
    pub(crate) fn create_from_sides(
        top: Side,
        bottom: Side,
        front: Side,
        right: Side,
        back: Side,
        left: Side,
    ) -> Self {
        let boxed_top = Box::new(top);
        let boxed_bottom = Box::new(bottom);
        let boxed_front = Box::new(front);
        let boxed_right = Box::new(right);
        let boxed_back = Box::new(back);
        let boxed_left = Box::new(left);
        Self {
            side_map: enum_map! {
                F::Top => boxed_top.clone(),
                F::Bottom => boxed_bottom.clone(),
                F::Front => boxed_front.clone(),
                F::Right => boxed_right.clone(),
                F::Back => boxed_back.clone(),
                F::Left => boxed_left.clone(),
            },
        }
    }
}

fn create_side(
    side_length: usize,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    let mut side = vec![];
    for _outer in 0..side_length {
        let inner_vec = vec![colour_variant_creator(None); side_length];
        side.push(inner_vec);
    }
    side
}

fn create_side_with_unique_characters(
    side_length: usize,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieFace,
) -> Side {
    assert!(
        (1..=8).contains(&side_length),
        "create_side_with_unique_characters does not support side_length > 8"
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

fn get_clockwise_slice_of_side(side: &Side, index_alignment: &IA) -> Vec<CubieFace> {
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

#[cfg(test)]
mod tests {
    use crate::{create_cube_from_sides, create_cube_side};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_3x3_cube() {
        let cube = Cube::create(3);

        let expected_cube = create_cube_from_sides!(
            top: create_cube_side!(White; 3),
            bottom: create_cube_side!(Yellow; 3),
            front: create_cube_side!(Blue; 3),
            right: create_cube_side!(Orange; 3),
            back: create_cube_side!(Green; 3),
            left: create_cube_side!(Red; 3),
        );

        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_unique_chars_3x3_cube() {
        let cube = Cube::create_with_unique_characters(3);

        let expected_cube = create_cube_from_sides!(
            top: vec![
                vec![CubieFace::White(Some('0')), CubieFace::White(Some('1')), CubieFace::White(Some('2'))],
                vec![CubieFace::White(Some('3')), CubieFace::White(Some('4')), CubieFace::White(Some('5'))],
                vec![CubieFace::White(Some('6')), CubieFace::White(Some('7')), CubieFace::White(Some('8'))],
            ],
            bottom: vec![
                vec![CubieFace::Yellow(Some('0')), CubieFace::Yellow(Some('1')), CubieFace::Yellow(Some('2'))],
                vec![CubieFace::Yellow(Some('3')), CubieFace::Yellow(Some('4')), CubieFace::Yellow(Some('5'))],
                vec![CubieFace::Yellow(Some('6')), CubieFace::Yellow(Some('7')), CubieFace::Yellow(Some('8'))],
            ],
            front: vec![
                vec![CubieFace::Blue(Some('0')), CubieFace::Blue(Some('1')), CubieFace::Blue(Some('2'))],
                vec![CubieFace::Blue(Some('3')), CubieFace::Blue(Some('4')), CubieFace::Blue(Some('5'))],
                vec![CubieFace::Blue(Some('6')), CubieFace::Blue(Some('7')), CubieFace::Blue(Some('8'))],
            ],
            right: vec![
                vec![CubieFace::Orange(Some('0')), CubieFace::Orange(Some('1')), CubieFace::Orange(Some('2'))],
                vec![CubieFace::Orange(Some('3')), CubieFace::Orange(Some('4')), CubieFace::Orange(Some('5'))],
                vec![CubieFace::Orange(Some('6')), CubieFace::Orange(Some('7')), CubieFace::Orange(Some('8'))],
            ],
            back: vec![
                vec![CubieFace::Green(Some('0')), CubieFace::Green(Some('1')), CubieFace::Green(Some('2'))],
                vec![CubieFace::Green(Some('3')), CubieFace::Green(Some('4')), CubieFace::Green(Some('5'))],
                vec![CubieFace::Green(Some('6')), CubieFace::Green(Some('7')), CubieFace::Green(Some('8'))],
            ],
            left: vec![
                vec![CubieFace::Red(Some('0')), CubieFace::Red(Some('1')), CubieFace::Red(Some('2'))],
                vec![CubieFace::Red(Some('3')), CubieFace::Red(Some('4')), CubieFace::Red(Some('5'))],
                vec![CubieFace::Red(Some('6')), CubieFace::Red(Some('7')), CubieFace::Red(Some('8'))],
            ],
        );

        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_default_3x3_cube_debug_repr() {
        let cube = Cube::create(3);

        let output = format!("{:?}", cube);

        let expected_output = format!(
            r#"      {0} {0} {0}
      {0} {0} {0}
      {0} {0} {0}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
      {5} {5} {5}
      {5} {5} {5}
      {5} {5} {5}
"#,
            CubieFace::White(None).get_coloured_display_char(),
            CubieFace::Red(None).get_coloured_display_char(),
            CubieFace::Blue(None).get_coloured_display_char(),
            CubieFace::Orange(None).get_coloured_display_char(),
            CubieFace::Green(None).get_coloured_display_char(),
            CubieFace::Yellow(None).get_coloured_display_char(),
        );

        assert_eq!(expected_output, output);
    }
}
