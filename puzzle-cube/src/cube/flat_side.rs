use anyhow::Context as _;

use super::cubie_face::CubieFace;

/// A representation of a cube side that uses a single [`Vec`] to support dynamic sizing at runtime, with better
/// performance than a naive nested `Vec<Vec<CubieFace>>` approach, while providing an API that 'looks like' a
/// `Vec<Vec<CubieFace>>` to some extent. For example, returning a row is easier than returning a column so the
/// types don't match.
///
/// The single [`Vec`] stores the side in row-major order.
#[derive(Debug, PartialEq)]
pub struct FlatSide {
    side_length: usize,
    inner: Vec<CubieFace>,
}

impl FlatSide {
    /// Create a new [`FlatSide`] instance from an existing [`Vec`].
    ///
    /// Note that the length of `inner` must be equal to `side_length` squared.
    pub fn new(side_length: usize, inner: Vec<CubieFace>) -> Self {
        assert_eq!(side_length * side_length, inner.len()); // todo return opt?
        Self { side_length, inner }
    }

    /// Get the side length.
    ///
    /// Note this is _not_ the length of the inner single [`Vec`].
    pub fn side_length(&self) -> usize {
        self.side_length
    }

    /// Get row index `y` of the side.
    pub fn row(&self, y: usize) -> Option<&[CubieFace]> {
        // todo return double ended iterator? callers clone anyway- one also reverses the slice
        if y >= self.side_length {
            return None;
        }

        let start_i = self.side_length * y;
        let end_i_excl = start_i + self.side_length;
        Some(&self.inner[start_i..end_i_excl])
    }

    /// Get a mutable reference to row index `y` of the side.
    pub fn row_mut(&mut self, y: usize) -> Option<&mut [CubieFace]> {
        if y >= self.side_length {
            return None;
        }

        let start_i = self.side_length * y;
        let end_i_excl = start_i + self.side_length;
        Some(&mut self.inner[start_i..end_i_excl])
    }

    /// Iterate all [`CubieFace`]s of this side grouped by row.
    pub fn rows(&self) -> impl Iterator<Item = &[CubieFace]> {
        self.inner.chunks(self.side_length)
    }

    /// Replace the values in row index `y` with the values in `values`.
    pub fn clone_row_from(&mut self, y: usize, values: &[CubieFace]) -> anyhow::Result<()> {
        self.row_mut(y)
            .with_context(|| format!("side did not have requested layer ({y})"))?
            .clone_from_slice(values);
        Ok(())
    }

    /// Get column index `x` of the side.
    pub fn col(&self, x: usize) -> Option<impl DoubleEndedIterator<Item = &CubieFace>> {
        if x >= self.side_length {
            return None;
        }

        Some(self.inner.iter().skip(x).step_by(self.side_length))
    }

    /// Get mutable references to the [`CubieFace`]s of column `x` of the side.
    pub fn col_mut(&mut self, x: usize) -> Option<impl DoubleEndedIterator<Item = &mut CubieFace>> {
        if x >= self.side_length {
            return None;
        }

        Some(self.inner.iter_mut().skip(x).step_by(self.side_length))
    }

    /// Replace the values in column index `x` with the values in `values`.
    pub fn clone_col_from(&mut self, x: usize, values: &[CubieFace]) -> anyhow::Result<()> {
        self.col_mut(x)
            .with_context(|| format!("side did not have requested layer ({x})"))?
            .zip(values.iter())
            .for_each(|(column_entry, new_value)| *column_entry = *new_value);
        Ok(())
    }

    /// Iterate all [`CubieFace`]s of this side in row-major order.
    pub fn iter_flat(&self) -> impl Iterator<Item = &CubieFace> {
        self.inner.iter()
    }

    /// Rotate this side in place, 90 degrees clockwise.
    pub fn rotate_90_degrees_clockwise(&mut self) {
        self.reverse_order_of_rows();

        for i in 1..self.side_length {
            (0..i).for_each(|j| {
                let left_y = j;
                let left_x = i;
                let left_i = left_y * self.side_length + left_x;

                let right_y = i;
                let right_x = j;
                let right_i = right_y * self.side_length + right_x;

                let small_i = left_i.min(right_i);
                let big_i = left_i.max(right_i);
                let (left, right) = self.inner.split_at_mut(big_i);
                let left = &mut left[small_i..=small_i];
                let right = &mut right[0..=0];
                left.swap_with_slice(right);
            });
        }
    }

    /// The equivalent of `.reverse()` on the naive `Vec<Vec<CubieFace>>` implementation of a side.
    fn reverse_order_of_rows(&mut self) {
        let rows_to_swap = self.side_length / 2;

        for i in 0..rows_to_swap {
            let left_start = i * self.side_length;
            let left_end_excl = left_start + self.side_length;

            let right_end_excl = self.inner.len() - left_start;
            let right_start = right_end_excl - self.side_length;

            let (left, right) = self.inner.split_at_mut(left_end_excl);
            let left = &mut left[left_start..];

            let right_start = right_start - left_end_excl;
            let right_end_excl = right_end_excl - left_end_excl;
            let right = &mut right[right_start..right_end_excl];

            left.swap_with_slice(right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rotate_90_degrees_clockwise_4x4x4() {
        let mut side = FlatSide::new(
            4,
            (0..16)
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );
        side.rotate_90_degrees_clockwise();

        #[rustfmt::skip]
        let inner = [
            12, 8, 4, 0,
            13, 9, 5, 1,
            14, 10, 6, 2,
            15, 11, 7, 3
        ];
        let expected_side = FlatSide::new(
            4,
            inner
                .into_iter()
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );

        assert_eq!(expected_side, side);
    }

    #[test]
    fn rotate_90_degrees_clockwise_3x3x3() {
        let mut side = FlatSide::new(
            3,
            (0..9)
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );
        side.rotate_90_degrees_clockwise();

        #[rustfmt::skip]
        let inner = [
            6, 3, 0,
            7, 4, 1,
            8, 5, 2
        ];
        let expected_side = FlatSide::new(
            3,
            inner
                .into_iter()
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );

        assert_eq!(expected_side, side);
    }

    #[test]
    fn reverse_order_of_rows_4x4x4() {
        let mut side = FlatSide::new(
            4,
            (0..16)
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );
        side.reverse_order_of_rows();

        let expected_side = FlatSide::new(
            4,
            [(12..16), (8..12), (4..8), (0..4)]
                .into_iter()
                .flat_map(|range| {
                    range
                        .map(|n| n + u32::from(b'a'))
                        .map(|n| CubieFace::White(char::from_u32(n)))
                })
                .collect(),
        );

        assert_eq!(expected_side, side);
    }

    #[test]
    fn reverse_order_of_rows_3x3x3() {
        let mut side = FlatSide::new(
            3,
            (0..9)
                .map(|n| n + u32::from(b'a'))
                .map(|n| CubieFace::White(char::from_u32(n)))
                .collect(),
        );
        side.reverse_order_of_rows();

        let expected_side = FlatSide::new(
            3,
            [(6..9), (3..6), (0..3)]
                .into_iter()
                .flat_map(|range| {
                    range
                        .map(|n| n + u32::from(b'a'))
                        .map(|n| CubieFace::White(char::from_u32(n)))
                })
                .collect(),
        );

        assert_eq!(expected_side, side);
    }
}
