use rusty_puzzle_cube::cube::{
    DefaultSide, PuzzleCube,
    cubie_face::CubieFace,
    direction::Direction,
    face::{Face, IndexAlignment},
    rotation::{Rotation, RotationKind},
};
use three_d::{Instances, Mat4, Srgba};

use crate::gui::{
    anim_cube::HighlightState,
    colours::{
        BLUE_HIGHLIGHTED, GREEN_HIGHLIGHTED, ORANGE_HIGHLIGHTED, RED_HIGHLIGHTED,
        WHITE_HIGHLIGHTED, YELLOW_HIGHLIGHTED,
    },
};

use super::{
    anim_cube::{AnimCube, AnimationProgress, AnimationState},
    colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    transforms::{
        QUARTER_TURN, cubie_face_to_backing_transformation, cubie_face_to_transformation,
        fraction_of_quarter_turn,
    },
};

pub(crate) trait PuzzleCube3D: PuzzleCube {
    fn as_instances(&self) -> Instances;
    fn rotate_seq_with_progress(
        &mut self,
        rotations: impl ExactSizeIterator<Item = Rotation> + 'static,
    ) -> anyhow::Result<()>;
    fn animation_progress(&self) -> Option<&AnimationProgress>;
    fn cancel_animation(&mut self);
}

macro_rules! all_faces_to_instances {
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident, $highlight_state:ident) => {{
        let (iter_transformations, iter_colours) = all_faces_to_instances!(
            $cube,
            $side_length,
            $rotation_with_anim_transform,
            $highlight_state,
            Face::Front,
            Face::Back,
            Face::Left,
            Face::Right,
            Face::Up,
            Face::Down,
        );

        let required_capacity = 6 * $side_length * $side_length;
        let mut transformations = Vec::with_capacity(required_capacity);
        transformations.extend(iter_transformations);
        let mut colours = Vec::with_capacity(required_capacity);
        colours.extend(iter_colours);

        (transformations, colours)
    }};
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident, $highlight_state:ident, $this_face:expr) => {
        $crate::gui::cube_3d_ext::face_to_instances($this_face, $cube, $side_length, $rotation_with_anim_transform, $highlight_state)
    };
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident, $highlight_state:ident, $this_face:expr, $($tail:expr),+ $(,)?) => {{
        let (transforms, colours) = all_faces_to_instances!($cube, $side_length, $rotation_with_anim_transform, $highlight_state, $this_face);
        let (tail_transforms, tail_colours) = all_faces_to_instances!($cube, $side_length, $rotation_with_anim_transform, $highlight_state, $($tail),*);
        (
            transforms.chain(tail_transforms),
            colours.chain(tail_colours),
        )
    }};
}

impl<C: PuzzleCube<Side = DefaultSide>> PuzzleCube3D for AnimCube<C> {
    fn as_instances(&self) -> three_d::Instances {
        let cube = self;
        let side_length = self.side_length();
        let rotation_with_anim_transform = choose_anim_transform(&self.animation);
        let highlight_state = &self.highlight;
        let (transformations, colours) = all_faces_to_instances!(
            cube,
            side_length,
            rotation_with_anim_transform,
            highlight_state
        );
        Instances {
            transformations,
            colors: Some(colours),
            ..Default::default()
        }
    }

    fn rotate_seq_with_progress(
        &mut self,
        rotations: impl ExactSizeIterator<Item = Rotation> + 'static,
    ) -> anyhow::Result<()> {
        let mut rotations = rotations.into_iter();
        let sequence_total = Some(rotations.len());
        if let Some(rotation) = rotations.next() {
            self.animation = AnimationState::TransitioningToNext {
                rotation,
                progress: AnimationProgress {
                    sequence_total,
                    ..Default::default()
                },
                seq: Some(Box::new(rotations)),
            };
        }
        Ok(())
    }

    fn animation_progress(&self) -> Option<&AnimationProgress> {
        match &self.animation {
            AnimationState::Rotating { progress, .. }
            | AnimationState::TransitioningToNext { progress, .. } => Some(progress),
            AnimationState::Stationary => None,
        }
    }

    fn cancel_animation(&mut self) {
        self.animation = AnimationState::Stationary;
    }
}

fn choose_anim_transform(animation: &AnimationState) -> Option<(Rotation, Mat4)> {
    match animation {
        AnimationState::Rotating {
            rotation, progress, ..
        } => {
            // Minus a full quarter turn as the cube has already set itself to the new positions that we want to slowly animate toward
            let rad = fraction_of_quarter_turn(progress.single_rotation_linear) - QUARTER_TURN;
            Some((
                *rotation,
                match rotation {
                    Rotation {
                        relative_to,
                        direction: Direction::Clockwise,
                        ..
                    } => match relative_to {
                        Face::Up => Mat4::from_angle_y(-rad),
                        Face::Down => Mat4::from_angle_y(rad),
                        Face::Front => Mat4::from_angle_z(-rad),
                        Face::Right => Mat4::from_angle_x(-rad),
                        Face::Back => Mat4::from_angle_z(rad),
                        Face::Left => Mat4::from_angle_x(rad),
                    },
                    Rotation {
                        relative_to,
                        direction: Direction::Anticlockwise,
                        ..
                    } => match relative_to {
                        Face::Up => Mat4::from_angle_y(rad),
                        Face::Down => Mat4::from_angle_y(-rad),
                        Face::Front => Mat4::from_angle_z(rad),
                        Face::Right => Mat4::from_angle_x(rad),
                        Face::Back => Mat4::from_angle_z(-rad),
                        Face::Left => Mat4::from_angle_x(-rad),
                    },
                },
            ))
        }
        AnimationState::Stationary | AnimationState::TransitioningToNext { .. } => None,
    }
}

fn face_to_instances<'a, C: PuzzleCube<Side = DefaultSide>>(
    face: Face,
    cube: &'a C,
    side_length: usize,
    rotation_with_anim_transform: Option<(Rotation, Mat4)>,
    highlight_state: &'a HighlightState,
) -> (
    impl Iterator<Item = Mat4> + 'a,
    impl Iterator<Item = Srgba> + 'a,
) {
    let transformations =
        cube.side(face)
            .iter()
            .flatten()
            .enumerate()
            .flat_map(move |(i, _cubie_face)| {
                let y = i / side_length;
                let x = i % side_length;

                let transform = cubie_face_to_transformation(side_length, face, x, y);
                let backing_transform =
                    cubie_face_to_backing_transformation(side_length, face, x, y);

                match rotation_with_anim_transform {
                    Some((rotation, anim_transform))
                        if should_apply_anim(face, side_length, x, y, rotation) =>
                    {
                        [
                            anim_transform * transform,
                            anim_transform * backing_transform,
                        ]
                    }
                    _ => [transform, backing_transform],
                }
            });

    let colours = cube
        .side(face)
        .iter()
        .flatten()
        .enumerate()
        .flat_map(move |(i, cubie_face)| {
            let y = side_length - i / side_length - 1; // gui has y axis flipped compared to lib
            let x = i % side_length;

            let should_highlight = matches!(highlight_state, HighlightState::Cubie(hovered) if hovered.face == face && hovered.col == x && hovered.row == y);

            [cubie_face_to_colour(*cubie_face, should_highlight), Srgba::BLACK]
        });

    (transformations, colours)
}

fn should_apply_anim(
    face: Face,
    side_length: usize,
    x: usize,
    y: usize,
    rotation: Rotation,
) -> bool {
    let opposite_end_minus_layer = |layer| side_length - 1 - layer;

    match rotation.kind {
        RotationKind::MultiSetback { start_layer: 0, .. }
        | RotationKind::Multilayer { .. }
        | RotationKind::Setback { layer: 0 }
        | RotationKind::FaceOnly
            if face == rotation.relative_to =>
        {
            true
        }
        RotationKind::Multilayer { layer }
            if face == !rotation.relative_to && layer == opposite_end_minus_layer(0) =>
        {
            true
        }
        RotationKind::MultiSetback { end_layer, .. }
            if face == !rotation.relative_to && end_layer == opposite_end_minus_layer(0) =>
        {
            true
        }
        _ => match rotation
            .relative_to
            .adjacent_faces_clockwise()
            .iter()
            .find(|(f, _)| f == &face)
        {
            Some((_, index_alignment)) => match rotation.kind {
                RotationKind::FaceOnly => match index_alignment {
                    IndexAlignment::OuterStart => x == 0,
                    IndexAlignment::OuterEnd => x == opposite_end_minus_layer(0),
                    IndexAlignment::InnerFirst => y == 0,
                    IndexAlignment::InnerLast => y == opposite_end_minus_layer(0),
                },
                RotationKind::Setback { layer } => match index_alignment {
                    IndexAlignment::OuterStart => x == layer,
                    IndexAlignment::OuterEnd => x == opposite_end_minus_layer(layer),
                    IndexAlignment::InnerFirst => y == layer,
                    IndexAlignment::InnerLast => y == opposite_end_minus_layer(layer),
                },
                RotationKind::Multilayer { layer } => match index_alignment {
                    IndexAlignment::OuterStart => x <= layer,
                    IndexAlignment::OuterEnd => x >= opposite_end_minus_layer(layer),
                    IndexAlignment::InnerFirst => y <= layer,
                    IndexAlignment::InnerLast => y >= opposite_end_minus_layer(layer),
                },
                RotationKind::MultiSetback {
                    start_layer,
                    end_layer,
                } => match index_alignment {
                    IndexAlignment::OuterStart => (start_layer..=end_layer).contains(&x),
                    IndexAlignment::OuterEnd => (opposite_end_minus_layer(end_layer)
                        ..=opposite_end_minus_layer(start_layer))
                        .contains(&x),
                    IndexAlignment::InnerFirst => (start_layer..=end_layer).contains(&y),
                    IndexAlignment::InnerLast => (opposite_end_minus_layer(end_layer)
                        ..=opposite_end_minus_layer(start_layer))
                        .contains(&y),
                },
            },
            _ => side_length == 1 && face == !rotation.relative_to,
        },
    }
}

fn cubie_face_to_colour(cubie_face: CubieFace, should_highlight: bool) -> Srgba {
    if should_highlight {
        match cubie_face {
            CubieFace::Blue(_) => BLUE_HIGHLIGHTED,
            CubieFace::Green(_) => GREEN_HIGHLIGHTED,
            CubieFace::Orange(_) => ORANGE_HIGHLIGHTED,
            CubieFace::Red(_) => RED_HIGHLIGHTED,
            CubieFace::White(_) => WHITE_HIGHLIGHTED,
            CubieFace::Yellow(_) => YELLOW_HIGHLIGHTED,
        }
    } else {
        match cubie_face {
            CubieFace::Blue(_) => BLUE,
            CubieFace::Green(_) => GREEN,
            CubieFace::Orange(_) => ORANGE,
            CubieFace::Red(_) => RED,
            CubieFace::White(_) => WHITE,
            CubieFace::Yellow(_) => YELLOW,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_cubie_face_to_colour_blue() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Blue(None), false),
            Srgba {
                r: 0,
                g: 0,
                b: 204,
                a: 255
            }
        );
    }

    #[test]
    fn test_cubie_face_to_colour_green() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Green(None), false),
            Srgba {
                r: 0,
                g: 204,
                b: 0,
                a: 255
            }
        );
    }

    #[test]
    fn test_cubie_face_to_colour_orange() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Orange(None), false),
            Srgba {
                r: 255,
                g: 125,
                b: 0,
                a: 255
            }
        );
    }

    #[test]
    fn test_cubie_face_to_colour_red() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Red(None), false),
            Srgba {
                r: 204,
                g: 0,
                b: 0,
                a: 255
            }
        );
    }

    #[test]
    fn test_cubie_face_to_colour_white() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::White(None), false),
            Srgba {
                r: 255,
                g: 255,
                b: 255,
                a: 255
            }
        );
    }

    #[test]
    fn test_cubie_face_to_colour_yellow() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Yellow(None), false),
            Srgba {
                r: 224,
                g: 224,
                b: 0,
                a: 255
            }
        );
    }
}
