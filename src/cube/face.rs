use enum_map::Enum;

#[derive(Enum)]
pub(crate) enum Face {
    Top,
    Bottom,
    Front,
    Right,
    Back,
    Left,
}
