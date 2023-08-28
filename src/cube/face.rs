use enum_map::Enum;
use Face::{Back, Bottom, Front, Left, Right, Top};
use IndexAlignment::{InnerFirst, InnerLast, OuterEnd, OuterStart};

#[derive(Debug, Clone, Copy, Enum)]
pub(crate) enum Face {
    Top,
    Bottom,
    Front,
    Right,
    Back,
    Left,
}

impl Face {
    pub(crate) fn adjacent_faces_clockwise(&self) -> [(Face, IndexAlignment); 4] {
        match self {
            Top => [
                (Front, InnerFirst),
                (Left, InnerFirst),
                (Back, InnerFirst),
                (Right, InnerFirst),
            ],
            Bottom => [
                (Front, InnerLast),
                (Right, InnerLast),
                (Back, InnerLast),
                (Left, InnerLast),
            ],
            Front => [
                (Top, InnerLast),
                (Right, OuterStart),
                (Bottom, InnerFirst),
                (Left, OuterEnd),
            ],
            Right => [
                (Top, OuterEnd),
                (Back, OuterStart),
                (Bottom, OuterEnd),
                (Front, OuterEnd),
            ],
            Back => [
                (Top, InnerFirst),
                (Left, OuterStart),
                (Bottom, InnerLast),
                (Right, OuterEnd),
            ],
            Left => [
                (Top, OuterStart),
                (Front, OuterStart),
                (Bottom, OuterStart),
                (Back, OuterEnd),
            ],
        }
    }
}

#[derive(Debug)]
pub(crate) enum IndexAlignment {
    /// A side is a Vec of Vec of CubieColour
    /// This enum describes an edge of the 2d side
    /// e.g.
    /// [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ]
    ///
    /// InnerFirst  = 0, 1, 2
    /// InnerLast   = 6, 7, 8
    /// OuterStart  = 0, 3, 6
    /// OuterEnd    = 2, 5, 8
    OuterStart,
    OuterEnd,
    InnerFirst,
    InnerLast,
}
