use enum_map::Enum;
use Face as F;
use IndexAlignment as IA;

#[derive(Debug, Clone, Copy, Enum, PartialEq)]
pub enum Face {
    Top,
    Bottom,
    Front,
    Right,
    Back,
    Left,
}

impl Face {
    pub(crate) fn adjacent_faces_clockwise(self) -> [(Face, IndexAlignment); 4] {
        match self {
            F::Top => [
                (F::Front, IA::InnerFirst),
                (F::Left, IA::InnerFirst),
                (F::Back, IA::InnerFirst),
                (F::Right, IA::InnerFirst),
            ],
            F::Bottom => [
                (F::Front, IA::InnerLast),
                (F::Right, IA::InnerLast),
                (F::Back, IA::InnerLast),
                (F::Left, IA::InnerLast),
            ],
            F::Front => [
                (F::Top, IA::InnerLast),
                (F::Right, IA::OuterStart),
                (F::Bottom, IA::InnerFirst),
                (F::Left, IA::OuterEnd),
            ],
            F::Right => [
                (F::Top, IA::OuterEnd),
                (F::Back, IA::OuterStart),
                (F::Bottom, IA::OuterEnd),
                (F::Front, IA::OuterEnd),
            ],
            F::Back => [
                (F::Top, IA::InnerFirst),
                (F::Left, IA::OuterStart),
                (F::Bottom, IA::InnerLast),
                (F::Right, IA::OuterEnd),
            ],
            F::Left => [
                (F::Top, IA::OuterStart),
                (F::Front, IA::OuterStart),
                (F::Bottom, IA::OuterStart),
                (F::Back, IA::OuterEnd),
            ],
        }
    }
}

/// This enum describes an edge of the 2d side, where a side is a `Vec<Vec<CubieFace>>`.
///
/// For example, given a 3x3 side with numbers representing `CubieFace` instances:
///```text
/// [
///     [0, 1, 2],
///     [3, 4, 5],
///     [6, 7, 8],
/// ]
///```
/// Variants of this enum would represent the following slices:
/// ```
/// InnerFirst  = 0, 1, 2
/// InnerLast   = 6, 7, 8
/// OuterStart  = 0, 3, 6
/// OuterEnd    = 2, 5, 8
/// ```
#[derive(Debug, PartialEq)]
pub(crate) enum IndexAlignment {
    OuterStart,
    OuterEnd,
    InnerFirst,
    InnerLast,
}
