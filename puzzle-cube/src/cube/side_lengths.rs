/// A valid side length for a `Cube` that has no unique display chars.
///
/// The provided `side_length` here must be >= 1.
#[derive(Copy, Clone)]
pub struct SideLength(usize);

impl TryFrom<usize> for SideLength {
    type Error = anyhow::Error;

    fn try_from(side_length: usize) -> Result<Self, Self::Error> {
        if (1..).contains(&side_length) {
            Ok(SideLength(side_length))
        } else {
            Err(anyhow::format_err!(
                "Cannot have a side length of less than 1"
            ))
        }
    }
}

impl From<SideLength> for usize {
    fn from(val: SideLength) -> Self {
        val.0
    }
}

/// A valid side length for a `Cube` that has unique display chars.
///
/// The provided `side_length` here must be >=1 and <=8 to allow for unique, visible characters per cubie in the basic ascii range.
#[derive(Copy, Clone)]
pub struct UniqueCharsSideLength(usize);

impl TryFrom<usize> for UniqueCharsSideLength {
    type Error = anyhow::Error;

    fn try_from(side_length: usize) -> Result<Self, Self::Error> {
        if (1..=8).contains(&side_length) {
            Ok(UniqueCharsSideLength(side_length))
        } else {
            Err(anyhow::format_err!("Cannot have a side length of less than 1, nor of greater than 8 when using unique display chars"))
        }
    }
}

impl From<UniqueCharsSideLength> for usize {
    fn from(val: UniqueCharsSideLength) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_length_valid() -> anyhow::Result<()> {
        for side_length in 1..=100 {
            let SideLength(held_value) = SideLength::try_from(side_length)?;
            assert_eq!(side_length, held_value);
        }
        Ok(())
    }

    #[test]
    fn test_side_length_invalid() {
        assert!(SideLength::try_from(0).is_err());
    }

    #[test]
    fn test_unique_chars_side_length_valid() -> anyhow::Result<()> {
        for side_length in 1..=8 {
            let UniqueCharsSideLength(held_value) = UniqueCharsSideLength::try_from(side_length)?;
            assert_eq!(side_length, held_value);
        }
        Ok(())
    }

    #[test]
    fn test_unique_chars_side_length_invalid() {
        assert!(UniqueCharsSideLength::try_from(0).is_err());

        for side_length in 9..=20 {
            assert!(UniqueCharsSideLength::try_from(side_length).is_err());
        }
    }
}
