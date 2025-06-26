use std::{fmt, mem};

use anyhow::Context;
use itertools::izip;

use self::cubie_face::CubieFace;
use self::direction::Direction;
use self::face::{Face as F, IndexAlignment as IA};
use self::helpers::{
    create_side, create_side_with_unique_characters, get_clockwise_slice_of_side_setback,
};
use self::rotation::{Rotation, RotationKind};
use self::side_lengths::{SideLength, UniqueCharsSideLength};

mod helpers;

/// An enum representing clockwise and anti-clockwise directions for a rotation.
pub mod direction;

/// An enum representing an individual cubie within one side of the cube, hence it only represents one face of the cubie.
pub mod cubie_face;

/// An enum representing the faces of a cube, and providing a mapping for 'adjacents' and `IndexAlignment` that are used to perform rotations of a face.
pub mod face;

/// Macros that aid in creating custom cube states for test cases.
pub mod macros;

/// Module defining the Rotation type that represents a single 90° rotation of some part of a cube.
pub mod rotation;

/// Structs that ensure cubes are constructed with only valid values for side length, depending on the type of cube.
pub mod side_lengths;

const HORIZONTAL_PADDING: &str = " ";

/// A representation of a cube that can be manipulated via making pre-defined rotations.
pub trait PuzzleCube {
    /// A type that holds the cubies currently on a face.
    type Side;

    /// Returns a new cube of this type in the default state at the given size.
    #[must_use]
    fn recreate_at_size(&self, side_length: SideLength) -> Self;

    /// Returns the amount of cubies along each edge of this cube.
    #[must_use]
    fn side_length(&self) -> usize;

    /// Returns a given face of the cube data structure to allow fully custom rendering of the cube.
    #[must_use]
    fn side(&self, face: F) -> &Self::Side;

    /// Perform `moves` random single-slice rotations on the cube.
    fn shuffle(&mut self, moves: usize) {
        for _ in 0..moves {
            let _ = self.rotate(Rotation::random(self.side_length()));
        }
    }

    /// Perform the given rotation once.
    /// ```no_run
    /// use rusty_puzzle_cube::cube::{Cube, PuzzleCube as _, face::Face, rotation::Rotation};
    ///
    /// let mut cube = Cube::default();
    /// cube.rotate(Rotation::clockwise(Face::Front));
    /// ```
    /// # Errors
    /// Err can only be returned if the given rotation is invalid for this cube.
    fn rotate(&mut self, rotation: Rotation) -> anyhow::Result<()>;

    /// Iterate over `rotations` performing each rotation encountered once.
    /// ```no_run
    /// use rusty_puzzle_cube::cube::{Cube, PuzzleCube as _, face::Face, rotation::Rotation};
    ///
    /// let mut cube = Cube::default();
    /// cube.rotate_seq(vec![Rotation::clockwise(Face::Front), Rotation::anticlockwise(Face::Right)]);
    /// ```
    /// # Errors
    /// Err can only be returned if any of the given rotations are invalid for this cube.
    fn rotate_seq(
        &mut self,
        rotations: impl IntoIterator<Item = Rotation> + 'static,
    ) -> anyhow::Result<()> {
        for r in rotations {
            self.rotate(r)?;
        }
        Ok(())
    }
}

/// The `Side` type, for the provided `Cube`'s implementation of `PuzzleCube`, that holds the cubies currently on a face.
pub type DefaultSide = Vec<Vec<CubieFace>>;

/// An implementer of the `PuzzleCube` trait.
#[derive(PartialEq)]
pub struct Cube {
    side_length: usize,
    up: DefaultSide,
    down: DefaultSide,
    front: DefaultSide,
    right: DefaultSide,
    back: DefaultSide,
    left: DefaultSide,
}

impl PuzzleCube for Cube {
    type Side = DefaultSide;

    fn recreate_at_size(&self, side_length: SideLength) -> Self {
        Cube::create(side_length)
    }

    fn side_length(&self) -> usize {
        self.side_length
    }

    fn side(&self, face: F) -> &Self::Side {
        match face {
            F::Up => &self.up,
            F::Down => &self.down,
            F::Front => &self.front,
            F::Right => &self.right,
            F::Back => &self.back,
            F::Left => &self.left,
        }
    }

    fn rotate(&mut self, rotation: Rotation) -> anyhow::Result<()> {
        let rotation = rotation.normalise(self.side_length);

        match rotation {
            Rotation {
                direction: Direction::Anticlockwise,
                ..
            } => {
                let reversed = !rotation;
                self.rotate(reversed)?;
                self.rotate(reversed)?;
                self.rotate(reversed)?;
            }
            Rotation {
                relative_to,
                direction: Direction::Clockwise,
                kind: RotationKind::FaceOnly,
            } => {
                self.rotate_layer(relative_to, 0)?;
            }
            Rotation {
                relative_to,
                direction: Direction::Clockwise,
                kind: RotationKind::Setback { layer },
            } => {
                self.rotate_layer(relative_to, layer)?;
            }
            r @ Rotation {
                direction: Direction::Clockwise,
                kind: RotationKind::Multilayer { layer },
                ..
            } => {
                for layer in 0..=layer {
                    self.rotate(Rotation {
                        kind: RotationKind::Setback { layer },
                        ..r
                    })?;
                }
            }
            r @ Rotation {
                direction: Direction::Clockwise,
                kind:
                    RotationKind::MultiSetback {
                        start_layer,
                        end_layer,
                    },
                ..
            } => {
                for layer in start_layer..=end_layer {
                    self.rotate(Rotation {
                        kind: RotationKind::Setback { layer },
                        ..r
                    })?;
                }
            }
        }
        Ok(())
    }
}

impl Cube {
    /// Create a new `Cube` instance with `side_length` cubies along each edge.
    /// ```no_run
    /// use rusty_puzzle_cube::cube::{Cube, side_lengths::SideLength};
    ///
    /// let side_length = SideLength::try_from(5)?;
    /// let cube = Cube::create(side_length);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    #[must_use]
    pub fn create(side_length: SideLength) -> Self {
        Self {
            side_length: side_length.into(),
            up: create_side(side_length, &CubieFace::White),
            down: create_side(side_length, &CubieFace::Yellow),
            front: create_side(side_length, &CubieFace::Blue),
            right: create_side(side_length, &CubieFace::Orange),
            back: create_side(side_length, &CubieFace::Green),
            left: create_side(side_length, &CubieFace::Red),
        }
    }

    /// Create a new `Cube` instance with `side_length` cubies along each edge, where each cubie of a given colour has a unique character to represent it.
    ///
    /// This can be useful for printing out the cube to terminal to check that moves being made are exactly as expect, not just the same colours as we expect.
    #[must_use]
    pub fn create_with_unique_characters(side_length: UniqueCharsSideLength) -> Self {
        Self {
            side_length: side_length.into(),
            up: create_side_with_unique_characters(side_length, &CubieFace::White),
            down: create_side_with_unique_characters(side_length, &CubieFace::Yellow),
            front: create_side_with_unique_characters(side_length, &CubieFace::Blue),
            right: create_side_with_unique_characters(side_length, &CubieFace::Orange),
            back: create_side_with_unique_characters(side_length, &CubieFace::Green),
            left: create_side_with_unique_characters(side_length, &CubieFace::Red),
        }
    }

    fn side_mut(&mut self, face: F) -> &mut DefaultSide {
        match face {
            F::Up => &mut self.up,
            F::Down => &mut self.down,
            F::Front => &mut self.front,
            F::Right => &mut self.right,
            F::Back => &mut self.back,
            F::Left => &mut self.left,
        }
    }

    fn rotate_layer(&mut self, face: F, layers_back: usize) -> anyhow::Result<()> {
        if layers_back == 0 {
            self.rotate_face_90_degrees_clockwise_without_adjacents(face);
        }
        self.rotate_adjacents_90_deg_clockwise_setback(face, layers_back)
    }

    fn rotate_face_90_degrees_clockwise_without_adjacents(&mut self, face: F) {
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

    fn rotate_adjacents_90_deg_clockwise_setback(
        &mut self,
        face: F,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let adjacents = face.adjacent_faces_clockwise();
        let slice_0 = get_clockwise_slice_of_side_setback(
            self.side(adjacents[0].0),
            &adjacents[0].1,
            layers_back,
        )?;
        let slice_1 = get_clockwise_slice_of_side_setback(
            self.side(adjacents[1].0),
            &adjacents[1].1,
            layers_back,
        )?;
        let slice_2 = get_clockwise_slice_of_side_setback(
            self.side(adjacents[2].0),
            &adjacents[2].1,
            layers_back,
        )?;
        let slice_3 = get_clockwise_slice_of_side_setback(
            self.side(adjacents[3].0),
            &adjacents[3].1,
            layers_back,
        )?;

        let final_order = {
            let mut preliminary_order = adjacents.iter();
            let first_element = preliminary_order.next();
            preliminary_order
                .chain(first_element)
                .collect::<Vec<&(F, IA)>>()
        };

        self.copy_setback_adjacent_over(final_order[0], slice_0, layers_back)?;
        self.copy_setback_adjacent_over(final_order[1], slice_1, layers_back)?;
        self.copy_setback_adjacent_over(final_order[2], slice_2, layers_back)?;
        self.copy_setback_adjacent_over(final_order[3], slice_3, layers_back)?;

        Ok(())
    }

    fn copy_setback_adjacent_over(
        &mut self,
        (target_face, target_alignment): &(F, IA),
        unadjusted_values: Vec<CubieFace>,
        layers_back: usize,
    ) -> anyhow::Result<()> {
        let values = match target_alignment {
            IA::OuterEnd | IA::InnerFirst => {
                let mut new_values = unadjusted_values.clone();
                new_values.reverse();
                new_values
            }
            IA::OuterStart | IA::InnerLast => unadjusted_values,
        };

        let side_length = self.side_length;
        let side = self.side_mut(*target_face);
        match target_alignment {
            IA::OuterStart | IA::OuterEnd => {
                let inner_index = if *target_alignment == IA::OuterStart {
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
                let outer_index = if *target_alignment == IA::InnerFirst {
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

    fn write_indented_single_side(&self, f: &mut fmt::Formatter, face: F) -> fmt::Result {
        for cubie_row in self.side(face) {
            write!(
                f,
                "{}",
                format!(" {HORIZONTAL_PADDING}").repeat(self.side_length)
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
        let side_a = self.side(face_a).iter();
        let side_b = self.side(face_b).iter();
        let side_c = self.side(face_c).iter();
        let side_d = self.side(face_d).iter();

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

    fn print_to_formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented_single_side(f, F::Up)?;
        self.write_unindented_four_sides(f, F::Left, F::Front, F::Right, F::Back)?;
        self.write_indented_single_side(f, F::Down)?;
        Ok(())
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::create(
            3.try_into()
                .expect("3 is a known good value for side length"),
        )
    }
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print_to_formatter(f)?;
        Ok(())
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_to_formatter(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod impl_for_tests_only {
    use super::*;

    macro_rules! assert_side_lengths {
        ($side_length:expr, $($side:expr),* $(,)?) => {
            $(
                assert_eq!($side_length, $side.len(),
                    "{} had outer length {} but was expected to have length {}",
                    stringify!($side), $side.len(), $side_length);
                $side
                    .iter()
                    .enumerate()
                    .for_each(|(index, inner)|
                        assert_eq!($side_length, inner.len(),
                            "{} had inner (index {}) length {} but was expected to have length {}",
                            stringify!($side), index, inner.len(), $side_length));
            )*
        };
    }

    impl Cube {
        /// Create a new `Cube` instance with pre-made `Side` instances, specifically for easily defining test cases
        #[must_use]
        pub fn create_from_sides(
            up: DefaultSide,
            down: DefaultSide,
            front: DefaultSide,
            right: DefaultSide,
            back: DefaultSide,
            left: DefaultSide,
        ) -> Self {
            let side_length = up.len();
            assert_side_lengths!(side_length, up, down, front, right, back, left);

            Self {
                side_length,
                up,
                down,
                front,
                right,
                back,
                left,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_cube_from_sides, create_cube_side};

    use super::face::Face;
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_recreate_at_size() -> anyhow::Result<()> {
        let mut shuffled_large_cube = Cube::create(SideLength::try_from(5)?);
        shuffled_large_cube.shuffle(25);

        let original_cube = Cube::create(SideLength::try_from(3)?);

        assert_eq!(
            original_cube,
            shuffled_large_cube.recreate_at_size(SideLength::try_from(3)?)
        );

        Ok(())
    }

    #[test]
    fn test_side_length_getter() {
        let cube = Cube::default();
        assert_eq!(cube.side_length, cube.side_length());
    }

    #[test]
    fn test_side_getter() {
        let cube = Cube::default();
        assert_eq!(&cube.up, cube.side(Face::Up));
        assert_eq!(&cube.down, cube.side(Face::Down));
        assert_eq!(&cube.front, cube.side(Face::Front));
        assert_eq!(&cube.right, cube.side(Face::Right));
        assert_eq!(&cube.back, cube.side(Face::Back));
        assert_eq!(&cube.left, cube.side(Face::Left));
    }

    #[test]
    fn test_default_3x3_cube() {
        let cube = Cube::default();

        let expected_cube = create_cube_from_sides!(
            up: create_cube_side!(White; 3),
            down: create_cube_side!(Yellow; 3),
            front: create_cube_side!(Blue; 3),
            right: create_cube_side!(Orange; 3),
            back: create_cube_side!(Green; 3),
            left: create_cube_side!(Red; 3),
        );

        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_unique_chars_3x3_cube() {
        let cube = Cube::create_with_unique_characters(3.try_into().expect("known good value"));

        let expected_cube = create_cube_from_sides!(
            up: vec![
                vec![CubieFace::White(Some('0')), CubieFace::White(Some('1')), CubieFace::White(Some('2'))],
                vec![CubieFace::White(Some('3')), CubieFace::White(Some('4')), CubieFace::White(Some('5'))],
                vec![CubieFace::White(Some('6')), CubieFace::White(Some('7')), CubieFace::White(Some('8'))],
            ],
            down: vec![
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
    fn test_default_3x3_cube_display_and_debug_repr() {
        let cube = Cube::default();

        let display_output = format!("{cube}");
        let debug_output = format!("{cube:?}");

        let expected_output = format!(
            r"      {0} {0} {0}
      {0} {0} {0}
      {0} {0} {0}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
{1} {1} {1} {2} {2} {2} {3} {3} {3} {4} {4} {4}
      {5} {5} {5}
      {5} {5} {5}
      {5} {5} {5}
",
            CubieFace::White(None).get_coloured_display_char(),
            CubieFace::Red(None).get_coloured_display_char(),
            CubieFace::Blue(None).get_coloured_display_char(),
            CubieFace::Orange(None).get_coloured_display_char(),
            CubieFace::Green(None).get_coloured_display_char(),
            CubieFace::Yellow(None).get_coloured_display_char(),
        );

        assert_eq!(expected_output, display_output);
        assert_eq!(expected_output, debug_output);
    }

    #[test]
    fn shuffle() {
        let mut cube = Cube::default();
        let original_cube = Cube::default();
        assert_eq!(
            original_cube, cube,
            "Test only works if cubes start out the same"
        );

        cube.shuffle(25);
        assert_ne!(original_cube, cube);
    }

    #[test]
    fn rotate_face() -> anyhow::Result<()> {
        let mut cube = Cube::create_with_unique_characters(2.try_into()?);
        cube.rotate(Rotation::anticlockwise(Face::Back))?;

        let expected_cube = create_cube_from_sides!(
            up: vec![
                vec![CubieFace::Red(Some('2')), CubieFace::Red(Some('0'))],
                vec![CubieFace::White(Some('2')), CubieFace::White(Some('3'))],
            ],
            down: vec![
                vec![CubieFace::Yellow(Some('0')), CubieFace::Yellow(Some('1'))],
                vec![CubieFace::Orange(Some('3')), CubieFace::Orange(Some('1'))],
            ],
            front: vec![
                vec![CubieFace::Blue(Some('0')), CubieFace::Blue(Some('1'))],
                vec![CubieFace::Blue(Some('2')), CubieFace::Blue(Some('3'))],
            ],
            right: vec![
                vec![CubieFace::Orange(Some('0')), CubieFace::White(Some('0'))],
                vec![CubieFace::Orange(Some('2')), CubieFace::White(Some('1'))],
            ],
            back: vec![
                vec![CubieFace::Green(Some('1')), CubieFace::Green(Some('3'))],
                vec![CubieFace::Green(Some('0')), CubieFace::Green(Some('2'))],
            ],
            left: vec![
                vec![CubieFace::Yellow(Some('2')), CubieFace::Red(Some('1'))],
                vec![CubieFace::Yellow(Some('3')), CubieFace::Red(Some('3'))],
            ],
        );
        assert_eq!(expected_cube, cube);
        Ok(())
    }

    #[test]
    fn rotate_inner() -> anyhow::Result<()> {
        let mut cube = Cube::create_with_unique_characters(3.try_into()?);
        cube.rotate(Rotation::clockwise_setback_from(Face::Front, 1))?;

        let expected_cube = create_cube_from_sides!(
            up: vec![
                vec![CubieFace::White(Some('0')), CubieFace::White(Some('1')), CubieFace::White(Some('2'))],
                vec![CubieFace::Red(Some('7')), CubieFace::Red(Some('4')), CubieFace::Red(Some('1'))],
                vec![CubieFace::White(Some('6')), CubieFace::White(Some('7')), CubieFace::White(Some('8'))],
            ],
            down: vec![
                vec![CubieFace::Yellow(Some('0')), CubieFace::Yellow(Some('1')), CubieFace::Yellow(Some('2'))],
                vec![CubieFace::Orange(Some('7')), CubieFace::Orange(Some('4')), CubieFace::Orange(Some('1'))],
                vec![CubieFace::Yellow(Some('6')), CubieFace::Yellow(Some('7')), CubieFace::Yellow(Some('8'))],
            ],
            front: vec![
                vec![CubieFace::Blue(Some('0')), CubieFace::Blue(Some('1')), CubieFace::Blue(Some('2'))],
                vec![CubieFace::Blue(Some('3')), CubieFace::Blue(Some('4')), CubieFace::Blue(Some('5'))],
                vec![CubieFace::Blue(Some('6')), CubieFace::Blue(Some('7')), CubieFace::Blue(Some('8'))],
            ],
            right: vec![
                vec![CubieFace::Orange(Some('0')), CubieFace::White(Some('3')), CubieFace::Orange(Some('2'))],
                vec![CubieFace::Orange(Some('3')), CubieFace::White(Some('4')), CubieFace::Orange(Some('5'))],
                vec![CubieFace::Orange(Some('6')), CubieFace::White(Some('5')), CubieFace::Orange(Some('8'))],
            ],
            back: vec![
                vec![CubieFace::Green(Some('0')), CubieFace::Green(Some('1')), CubieFace::Green(Some('2'))],
                vec![CubieFace::Green(Some('3')), CubieFace::Green(Some('4')), CubieFace::Green(Some('5'))],
                vec![CubieFace::Green(Some('6')), CubieFace::Green(Some('7')), CubieFace::Green(Some('8'))],
            ],
            left: vec![
                vec![CubieFace::Red(Some('0')), CubieFace::Yellow(Some('3')), CubieFace::Red(Some('2'))],
                vec![CubieFace::Red(Some('3')), CubieFace::Yellow(Some('4')), CubieFace::Red(Some('5'))],
                vec![CubieFace::Red(Some('6')), CubieFace::Yellow(Some('5')), CubieFace::Red(Some('8'))],
            ],
        );
        assert_eq!(expected_cube, cube);
        Ok(())
    }

    #[test]
    fn rotate_multilayer() -> anyhow::Result<()> {
        let mut cube = Cube::create_with_unique_characters(3.try_into()?);
        cube.rotate(Rotation::clockwise_multilayer_from(Face::Front, 1))?;

        let expected_cube = create_cube_from_sides!(
            up: vec![
                vec![CubieFace::White(Some('0')), CubieFace::White(Some('1')), CubieFace::White(Some('2'))],
                vec![CubieFace::Red(Some('7')), CubieFace::Red(Some('4')), CubieFace::Red(Some('1'))],
                vec![CubieFace::Red(Some('8')), CubieFace::Red(Some('5')), CubieFace::Red(Some('2'))],
            ],
            down: vec![
                vec![CubieFace::Orange(Some('6')), CubieFace::Orange(Some('3')), CubieFace::Orange(Some('0'))],
                vec![CubieFace::Orange(Some('7')), CubieFace::Orange(Some('4')), CubieFace::Orange(Some('1'))],
                vec![CubieFace::Yellow(Some('6')), CubieFace::Yellow(Some('7')), CubieFace::Yellow(Some('8'))],
            ],
            front: vec![
                vec![CubieFace::Blue(Some('6')), CubieFace::Blue(Some('3')), CubieFace::Blue(Some('0'))],
                vec![CubieFace::Blue(Some('7')), CubieFace::Blue(Some('4')), CubieFace::Blue(Some('1'))],
                vec![CubieFace::Blue(Some('8')), CubieFace::Blue(Some('5')), CubieFace::Blue(Some('2'))],
            ],
            right: vec![
                vec![CubieFace::White(Some('6')), CubieFace::White(Some('3')), CubieFace::Orange(Some('2'))],
                vec![CubieFace::White(Some('7')), CubieFace::White(Some('4')), CubieFace::Orange(Some('5'))],
                vec![CubieFace::White(Some('8')), CubieFace::White(Some('5')), CubieFace::Orange(Some('8'))],
            ],
            back: vec![
                vec![CubieFace::Green(Some('0')), CubieFace::Green(Some('1')), CubieFace::Green(Some('2'))],
                vec![CubieFace::Green(Some('3')), CubieFace::Green(Some('4')), CubieFace::Green(Some('5'))],
                vec![CubieFace::Green(Some('6')), CubieFace::Green(Some('7')), CubieFace::Green(Some('8'))],
            ],
            left: vec![
                vec![CubieFace::Red(Some('0')), CubieFace::Yellow(Some('3')), CubieFace::Yellow(Some('0'))],
                vec![CubieFace::Red(Some('3')), CubieFace::Yellow(Some('4')), CubieFace::Yellow(Some('1'))],
                vec![CubieFace::Red(Some('6')), CubieFace::Yellow(Some('5')), CubieFace::Yellow(Some('2'))],
            ],
        );
        assert_eq!(expected_cube, cube);
        Ok(())
    }

    #[test]
    fn rotate_multisetback() -> anyhow::Result<()> {
        let mut cube = Cube::create_with_unique_characters(4.try_into()?);
        cube.rotate(Rotation::clockwise_multisetback_from(Face::Left, 1, 2))?;

        let expected_cube = create_cube_from_sides!(
            up: vec![
                vec![CubieFace::White(Some('0')), CubieFace::Green(Some('>')), CubieFace::Green(Some('=')), CubieFace::White(Some('3'))],
                vec![CubieFace::White(Some('4')), CubieFace::Green(Some(':')), CubieFace::Green(Some('9')), CubieFace::White(Some('7'))],
                vec![CubieFace::White(Some('8')), CubieFace::Green(Some('6')), CubieFace::Green(Some('5')), CubieFace::White(Some(';'))],
                vec![CubieFace::White(Some('<')), CubieFace::Green(Some('2')), CubieFace::Green(Some('1')), CubieFace::White(Some('?'))],
            ],
            down: vec![
                vec![CubieFace::Yellow(Some('0')), CubieFace::Blue(Some('1')), CubieFace::Blue(Some('2')), CubieFace::Yellow(Some('3'))],
                vec![CubieFace::Yellow(Some('4')), CubieFace::Blue(Some('5')), CubieFace::Blue(Some('6')), CubieFace::Yellow(Some('7'))],
                vec![CubieFace::Yellow(Some('8')), CubieFace::Blue(Some('9')), CubieFace::Blue(Some(':')), CubieFace::Yellow(Some(';'))],
                vec![CubieFace::Yellow(Some('<')), CubieFace::Blue(Some('=')), CubieFace::Blue(Some('>')), CubieFace::Yellow(Some('?'))],
            ],
            front: vec![
                vec![CubieFace::Blue(Some('0')), CubieFace::White(Some('1')), CubieFace::White(Some('2')), CubieFace::Blue(Some('3'))],
                vec![CubieFace::Blue(Some('4')), CubieFace::White(Some('5')), CubieFace::White(Some('6')), CubieFace::Blue(Some('7'))],
                vec![CubieFace::Blue(Some('8')), CubieFace::White(Some('9')), CubieFace::White(Some(':')), CubieFace::Blue(Some(';'))],
                vec![CubieFace::Blue(Some('<')), CubieFace::White(Some('=')), CubieFace::White(Some('>')), CubieFace::Blue(Some('?'))],
            ],
            right: vec![
                vec![CubieFace::Orange(Some('0')), CubieFace::Orange(Some('1')), CubieFace::Orange(Some('2')), CubieFace::Orange(Some('3'))],
                vec![CubieFace::Orange(Some('4')), CubieFace::Orange(Some('5')), CubieFace::Orange(Some('6')), CubieFace::Orange(Some('7'))],
                vec![CubieFace::Orange(Some('8')), CubieFace::Orange(Some('9')), CubieFace::Orange(Some(':')), CubieFace::Orange(Some(';'))],
                vec![CubieFace::Orange(Some('<')), CubieFace::Orange(Some('=')), CubieFace::Orange(Some('>')), CubieFace::Orange(Some('?'))],
            ],
            back: vec![
                vec![CubieFace::Green(Some('0')), CubieFace::Yellow(Some('>')), CubieFace::Yellow(Some('=')), CubieFace::Green(Some('3'))],
                vec![CubieFace::Green(Some('4')), CubieFace::Yellow(Some(':')), CubieFace::Yellow(Some('9')), CubieFace::Green(Some('7'))],
                vec![CubieFace::Green(Some('8')), CubieFace::Yellow(Some('6')), CubieFace::Yellow(Some('5')), CubieFace::Green(Some(';'))],
                vec![CubieFace::Green(Some('<')), CubieFace::Yellow(Some('2')), CubieFace::Yellow(Some('1')), CubieFace::Green(Some('?'))],
            ],
            left: vec![
                vec![CubieFace::Red(Some('0')), CubieFace::Red(Some('1')), CubieFace::Red(Some('2')), CubieFace::Red(Some('3'))],
                vec![CubieFace::Red(Some('4')), CubieFace::Red(Some('5')), CubieFace::Red(Some('6')), CubieFace::Red(Some('7'))],
                vec![CubieFace::Red(Some('8')), CubieFace::Red(Some('9')), CubieFace::Red(Some(':')), CubieFace::Red(Some(';'))],
                vec![CubieFace::Red(Some('<')), CubieFace::Red(Some('=')), CubieFace::Red(Some('>')), CubieFace::Red(Some('?'))],
            ],
        );
        assert_eq!(expected_cube, cube);
        Ok(())
    }

    #[test]
    fn rotate_far_opposite_face_as_if_it_were_inner() -> anyhow::Result<()> {
        let side_length = 5;

        let mut cube_under_test = Cube::create_with_unique_characters(side_length.try_into()?);
        cube_under_test.rotate(Rotation::clockwise_setback_from(
            Face::Right,
            side_length - 1,
        ))?;

        let mut expected_cube = Cube::create_with_unique_characters(side_length.try_into()?);
        expected_cube.rotate(Rotation::anticlockwise(Face::Left))?;

        assert_eq!(expected_cube, cube_under_test);
        Ok(())
    }

    #[test]
    fn rotate_with_invalid_layer() -> anyhow::Result<()> {
        let side_length = 4;
        let mut cube = Cube::create(side_length.try_into()?);

        let invalid_layer_index = side_length;
        let rotation = Rotation::clockwise_setback_from(Face::Up, invalid_layer_index);
        let result = cube.rotate(rotation);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            format!("{err:?}")
                .starts_with("side did not have required layer (4 of outer vec of side)")
        );
        Ok(())
    }

    #[test]
    fn rotate_seq() -> anyhow::Result<()> {
        let side_length = 4;
        let mut seq_cube = Cube::create(side_length.try_into()?);
        let mut no_seq_cube = Cube::create(side_length.try_into()?);

        let rot_1 = Rotation::clockwise(Face::Up);
        let rot_2 = Rotation::clockwise(Face::Left);
        let rot_3 = Rotation::anticlockwise(Face::Up);

        seq_cube.rotate_seq(vec![rot_1, rot_2, rot_3])?;
        no_seq_cube.rotate(rot_1)?;
        no_seq_cube.rotate(rot_2)?;
        no_seq_cube.rotate(rot_3)?;

        assert_eq!(no_seq_cube, seq_cube);
        Ok(())
    }
}
