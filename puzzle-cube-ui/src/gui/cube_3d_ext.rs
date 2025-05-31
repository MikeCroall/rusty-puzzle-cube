use rusty_puzzle_cube::cube::{
    PuzzleCube,
    cubie_face::CubieFace,
    direction::Direction,
    face::{Face, IndexAlignment},
    rotation::Rotation,
};
use three_d::{Instances, Mat4, Matrix4, Srgba};

use super::{
    anim_cube::{AnimCube, AnimationState},
    colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    transforms::{
        QUARTER_TURN, cubie_face_to_backing_transformation, cubie_face_to_transformation,
        fraction_of_quarter_turn,
    },
};

pub(crate) trait PuzzleCube3D: PuzzleCube {
    fn as_instances(&self) -> Instances;
    fn cancel_animation(&mut self);
}

macro_rules! all_faces_to_instances {
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident) => {{
        let (iter_transformations, iter_colours) = all_faces_to_instances!(
            $cube,
            $side_length,
            $rotation_with_anim_transform,
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
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident, $this_face:expr) => {
        $crate::gui::cube_3d_ext::face_to_instances($this_face, $cube.side($this_face), $side_length, $rotation_with_anim_transform)
    };
    ($cube:ident, $side_length:ident, $rotation_with_anim_transform:ident, $this_face:expr, $($tail:expr),+ $(,)?) => {{
        let (transforms, colours) = all_faces_to_instances!($cube, $side_length, $rotation_with_anim_transform, $this_face);
        let (tail_transforms, tail_colours) = all_faces_to_instances!($cube, $side_length, $rotation_with_anim_transform, $($tail),*);
        (
            transforms.chain(tail_transforms),
            colours.chain(tail_colours),
        )
    }};
}

impl<C: PuzzleCube> PuzzleCube3D for AnimCube<C> {
    fn as_instances(&self) -> three_d::Instances {
        let cube = self;
        let side_length = self.side_length();
        let rotation_with_anim_transform = choose_anim_transform(&self.animation);
        let (transformations, colours) =
            all_faces_to_instances!(cube, side_length, rotation_with_anim_transform);
        Instances {
            transformations,
            colors: Some(colours),
            ..Default::default()
        }
    }

    fn cancel_animation(&mut self) {
        self.animation = AnimationState::Stationary;
    }
}

fn choose_anim_transform(animation: &AnimationState) -> Option<(Rotation, Matrix4<f32>)> {
    match animation {
        AnimationState::Rotating {
            rotation,
            progress_linear,
            ..
        } => {
            // Minus a full quarter turn as the cube has already set itself to the new positions that we want to slowly animate toward
            let rad = fraction_of_quarter_turn(*progress_linear) - QUARTER_TURN;
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

fn face_to_instances(
    face: Face,
    side: &[Vec<CubieFace>],
    side_length: usize,
    rotation_with_anim_transform: Option<(Rotation, Matrix4<f32>)>,
) -> (
    impl Iterator<Item = Matrix4<f32>> + '_,
    impl Iterator<Item = Srgba> + '_,
) {
    let transformations = side
        .iter()
        .flatten()
        .enumerate()
        .flat_map(move |(i, _cubie_face)| {
            let y = i / side_length;
            let x = i % side_length;

            let transform = cubie_face_to_transformation(side_length, face, x, y);
            let backing_transform = cubie_face_to_backing_transformation(side_length, face, x, y);

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

    let colours = side
        .iter()
        .flatten()
        .flat_map(|cubie_face| [cubie_face_to_colour(*cubie_face), Srgba::BLACK]);

    (transformations, colours)
}

fn should_apply_anim(
    face: Face,
    side_length: usize,
    x: usize,
    y: usize,
    rotation: Rotation,
) -> bool {
    if face == rotation.relative_to && rotation.layer == 0 {
        return true;
    }

    let adjacents = rotation.relative_to.adjacent_faces_clockwise();
    if let Some((_, index_alignment)) = adjacents.iter().find(|(f, _)| f == &face) {
        return match index_alignment {
            IndexAlignment::OuterStart => x == rotation.layer,
            IndexAlignment::OuterEnd => x == side_length - 1 - rotation.layer,
            IndexAlignment::InnerFirst => y == rotation.layer,
            IndexAlignment::InnerLast => y == side_length - 1 - rotation.layer,
        };
    }

    side_length == 1 && face == !rotation.relative_to
}

fn cubie_face_to_colour(cubie_face: CubieFace) -> Srgba {
    match cubie_face {
        CubieFace::Blue(_) => BLUE,
        CubieFace::Green(_) => GREEN,
        CubieFace::Orange(_) => ORANGE,
        CubieFace::Red(_) => RED,
        CubieFace::White(_) => WHITE,
        CubieFace::Yellow(_) => YELLOW,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_cubie_face_to_colour_blue() {
        assert_eq!(
            cubie_face_to_colour(CubieFace::Blue(None)),
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
            cubie_face_to_colour(CubieFace::Green(None)),
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
            cubie_face_to_colour(CubieFace::Orange(None)),
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
            cubie_face_to_colour(CubieFace::Red(None)),
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
            cubie_face_to_colour(CubieFace::White(None)),
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
            cubie_face_to_colour(CubieFace::Yellow(None)),
            Srgba {
                r: 224,
                g: 224,
                b: 0,
                a: 255
            }
        );
    }
}
