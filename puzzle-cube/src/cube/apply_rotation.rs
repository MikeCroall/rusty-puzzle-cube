use std::mem;

use anyhow::Context;

use super::{
    Cube, CubieFace, DefaultSide, PuzzleCube as _,
    direction::Direction,
    face::{Face as F, IndexAlignment as IA},
};

impl Cube {
    pub(crate) fn rotate_layer(
        &mut self,
        face: F,
        direction: Direction,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        if layers_back == 0 {
            self.rotate_face_90_degrees_without_adjacents(face, direction);
        } else if layers_back == self.side_length - 1 {
            self.rotate_face_90_degrees_without_adjacents(!face, direction);
        }
        self.rotate_adjacents_90_deg_setback(face, direction, layers_back)
    }

    fn rotate_face_90_degrees_without_adjacents(&mut self, face: F, direction: Direction) {
        let side_length = self.side_length;
        let side = self.side_mut(face);
        match direction {
            Direction::Clockwise => side.reverse(),
            Direction::Anticlockwise => side.iter_mut().for_each(|row| row.reverse()),
        }
        for i in 1..side_length {
            let (left, right) = side.split_at_mut(i);
            (0..i).for_each(|j| {
                mem::swap(&mut left[j][i], &mut right[0][j]);
            });
        }
    }

    fn rotate_adjacents_90_deg_setback(
        &mut self,
        face: F,
        direction: Direction,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let adjacents = match direction {
            Direction::Clockwise => face.adjacent_faces_clockwise(),
            Direction::Anticlockwise => {
                let mut adjacents = face.adjacent_faces_clockwise();
                adjacents.reverse();
                adjacents
            }
        };

        let slices =
            adjacents.map(|adj| get_slice_of_side_setback(self.side(adj.0), adj.1, layers_back));

        adjacents
            .iter()
            .cycle()
            .skip(1)
            .zip(slices)
            .try_for_each(|(adjacent, slice)| {
                self.copy_setback_adjacent_over(*adjacent, slice?, layers_back)
            })
    }

    fn copy_setback_adjacent_over(
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

fn get_slice_of_side_setback(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::cubie_face::CubieFace;

    mod clockwise {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn rotate_face_90_degrees_without_adjacents_clockwise() -> anyhow::Result<()> {
            let mut cube = Cube::create_with_unique_characters(3.try_into()?);
            cube.rotate_face_90_degrees_without_adjacents(F::Front, Direction::Clockwise);
            let side = cube.side(F::Front);

            let expected_side = vec![
                vec![
                    CubieFace::Blue(Some('6')),
                    CubieFace::Blue(Some('3')),
                    CubieFace::Blue(Some('0')),
                ],
                vec![
                    CubieFace::Blue(Some('7')),
                    CubieFace::Blue(Some('4')),
                    CubieFace::Blue(Some('1')),
                ],
                vec![
                    CubieFace::Blue(Some('8')),
                    CubieFace::Blue(Some('5')),
                    CubieFace::Blue(Some('2')),
                ],
            ];

            assert_eq!(&expected_side, side);

            Ok(())
        }

        #[test]
        fn rotate_adjacents_90_deg_setback_clockwise() -> anyhow::Result<()> {
            let mut cube = Cube::create_with_unique_characters(3.try_into()?);
            cube.rotate_adjacents_90_deg_setback(F::Front, Direction::Clockwise, 0)?;

            let unchanged_cube = Cube::create_with_unique_characters(3.try_into()?);
            assert_eq!(unchanged_cube.side(F::Front), cube.side(F::Front), "front");
            assert_eq!(unchanged_cube.side(F::Back), cube.side(F::Back), "back");

            let expected_up = vec![
                vec![
                    CubieFace::White(Some('0')),
                    CubieFace::White(Some('1')),
                    CubieFace::White(Some('2')),
                ],
                vec![
                    CubieFace::White(Some('3')),
                    CubieFace::White(Some('4')),
                    CubieFace::White(Some('5')),
                ],
                vec![
                    CubieFace::Red(Some('8')),
                    CubieFace::Red(Some('5')),
                    CubieFace::Red(Some('2')),
                ],
            ];
            assert_eq!(&expected_up, cube.side(F::Up), "up");

            let expected_right = vec![
                vec![
                    CubieFace::White(Some('6')),
                    CubieFace::Orange(Some('1')),
                    CubieFace::Orange(Some('2')),
                ],
                vec![
                    CubieFace::White(Some('7')),
                    CubieFace::Orange(Some('4')),
                    CubieFace::Orange(Some('5')),
                ],
                vec![
                    CubieFace::White(Some('8')),
                    CubieFace::Orange(Some('7')),
                    CubieFace::Orange(Some('8')),
                ],
            ];
            assert_eq!(&expected_right, cube.side(F::Right), "right");

            let expected_down = vec![
                vec![
                    CubieFace::Orange(Some('6')),
                    CubieFace::Orange(Some('3')),
                    CubieFace::Orange(Some('0')),
                ],
                vec![
                    CubieFace::Yellow(Some('3')),
                    CubieFace::Yellow(Some('4')),
                    CubieFace::Yellow(Some('5')),
                ],
                vec![
                    CubieFace::Yellow(Some('6')),
                    CubieFace::Yellow(Some('7')),
                    CubieFace::Yellow(Some('8')),
                ],
            ];
            assert_eq!(&expected_down, cube.side(F::Down), "down");

            let expected_left = vec![
                vec![
                    CubieFace::Red(Some('0')),
                    CubieFace::Red(Some('1')),
                    CubieFace::Yellow(Some('0')),
                ],
                vec![
                    CubieFace::Red(Some('3')),
                    CubieFace::Red(Some('4')),
                    CubieFace::Yellow(Some('1')),
                ],
                vec![
                    CubieFace::Red(Some('6')),
                    CubieFace::Red(Some('7')),
                    CubieFace::Yellow(Some('2')),
                ],
            ];
            assert_eq!(&expected_left, cube.side(F::Left), "left");

            Ok(())
        }
    }

    mod anticlockwise {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn rotate_face_90_degrees_without_adjacents_anticlockwise() -> anyhow::Result<()> {
            let mut cube = Cube::create_with_unique_characters(3.try_into()?);
            cube.rotate_face_90_degrees_without_adjacents(F::Front, Direction::Anticlockwise);
            let side = cube.side(F::Front);

            let expected_side = vec![
                vec![
                    CubieFace::Blue(Some('2')),
                    CubieFace::Blue(Some('5')),
                    CubieFace::Blue(Some('8')),
                ],
                vec![
                    CubieFace::Blue(Some('1')),
                    CubieFace::Blue(Some('4')),
                    CubieFace::Blue(Some('7')),
                ],
                vec![
                    CubieFace::Blue(Some('0')),
                    CubieFace::Blue(Some('3')),
                    CubieFace::Blue(Some('6')),
                ],
            ];

            assert_eq!(&expected_side, side);

            Ok(())
        }

        #[test]
        fn rotate_adjacents_90_deg_setback_anticlockwise() -> anyhow::Result<()> {
            let mut cube = Cube::create_with_unique_characters(3.try_into()?);
            cube.rotate_adjacents_90_deg_setback(F::Front, Direction::Anticlockwise, 0)?;

            let unchanged_cube = Cube::create_with_unique_characters(3.try_into()?);
            assert_eq!(unchanged_cube.side(F::Front), cube.side(F::Front), "front");
            assert_eq!(unchanged_cube.side(F::Back), cube.side(F::Back), "back");

            let expected_up = vec![
                vec![
                    CubieFace::White(Some('0')),
                    CubieFace::White(Some('1')),
                    CubieFace::White(Some('2')),
                ],
                vec![
                    CubieFace::White(Some('3')),
                    CubieFace::White(Some('4')),
                    CubieFace::White(Some('5')),
                ],
                vec![
                    CubieFace::Orange(Some('0')),
                    CubieFace::Orange(Some('3')),
                    CubieFace::Orange(Some('6')),
                ],
            ];
            assert_eq!(&expected_up, cube.side(F::Up), "up");

            let expected_right = vec![
                vec![
                    CubieFace::Yellow(Some('2')),
                    CubieFace::Orange(Some('1')),
                    CubieFace::Orange(Some('2')),
                ],
                vec![
                    CubieFace::Yellow(Some('1')),
                    CubieFace::Orange(Some('4')),
                    CubieFace::Orange(Some('5')),
                ],
                vec![
                    CubieFace::Yellow(Some('0')),
                    CubieFace::Orange(Some('7')),
                    CubieFace::Orange(Some('8')),
                ],
            ];
            assert_eq!(&expected_right, cube.side(F::Right), "right");

            let expected_down = vec![
                vec![
                    CubieFace::Red(Some('2')),
                    CubieFace::Red(Some('5')),
                    CubieFace::Red(Some('8')),
                ],
                vec![
                    CubieFace::Yellow(Some('3')),
                    CubieFace::Yellow(Some('4')),
                    CubieFace::Yellow(Some('5')),
                ],
                vec![
                    CubieFace::Yellow(Some('6')),
                    CubieFace::Yellow(Some('7')),
                    CubieFace::Yellow(Some('8')),
                ],
            ];
            assert_eq!(&expected_down, cube.side(F::Down), "down");

            let expected_left = vec![
                vec![
                    CubieFace::Red(Some('0')),
                    CubieFace::Red(Some('1')),
                    CubieFace::White(Some('8')),
                ],
                vec![
                    CubieFace::Red(Some('3')),
                    CubieFace::Red(Some('4')),
                    CubieFace::White(Some('7')),
                ],
                vec![
                    CubieFace::Red(Some('6')),
                    CubieFace::Red(Some('7')),
                    CubieFace::White(Some('6')),
                ],
            ];
            assert_eq!(&expected_left, cube.side(F::Left), "left");

            Ok(())
        }
    }
}
