use rusty_puzzle_cube::cube::{cubie_face::CubieFace, face::Face, Cube};
use three_d::{Instances, Matrix4, Srgba};

use super::{
    colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    transforms::cubie_face_to_transformation,
};

pub(crate) trait ToInstances {
    fn to_instances(&self) -> Instances;
}

macro_rules! all_faces_to_instances {
    ($side_map:ident, $side_length:ident) => {{
        let (iter_transformations, iter_colours) = all_faces_to_instances!(
            $side_map,
            $side_length,
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
    ($side_map:ident, $side_length:ident, $this_face:expr) => {
        face_to_instances($this_face, &$side_map[$this_face], $side_length)
    };
    ($side_map:ident, $side_length:ident, $this_face:expr, $($tail:expr),+ $(,)?) => {{
        let (transforms, colours) = all_faces_to_instances!($side_map, $side_length, $this_face);
        let (tail_transforms, tail_colours) = all_faces_to_instances!($side_map, $side_length, $($tail),*);
        (
            transforms.chain(tail_transforms),
            colours.chain(tail_colours),
        )
    }};
}

impl ToInstances for Cube {
    fn to_instances(&self) -> Instances {
        let side_length = self.side_length();
        let side_map = self.side_map();
        let (transformations, colours) = all_faces_to_instances!(side_map, side_length);
        Instances {
            transformations,
            colors: Some(colours),
            ..Default::default()
        }
    }
}

fn face_to_instances(
    face: Face,
    side: &[Vec<CubieFace>],
    side_length: usize,
) -> (
    impl Iterator<Item = Matrix4<f32>> + '_,
    impl Iterator<Item = Srgba> + '_,
) {
    let transformations = side
        .iter()
        .flatten()
        .enumerate()
        .map(move |(i, _cubie_face)| {
            let y = i / side_length;
            let x = i % side_length;
            cubie_face_to_transformation(side_length, face, x, y)
        });

    let colours = side
        .iter()
        .flatten()
        .map(move |cubie_face| cubie_face_to_colour(*cubie_face));

    (transformations, colours)
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
                r: 224,
                g: 112,
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
