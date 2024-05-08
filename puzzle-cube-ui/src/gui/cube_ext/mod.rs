use rusty_puzzle_cube::cube::{cubie_face::CubieFace, face::Face, Cube, SideMap};
use three_d::{Instances, Matrix4, Srgba};

use crate::gui::{
    colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    transforms::{
        half_turn_around_y, position_from_origin_centered_to, quarter_turn_around_x,
        quarter_turn_around_y, rev_quarter_turn_around_x, rev_quarter_turn_around_y, scale_down,
        translate_away, translate_down, translate_left, translate_right, translate_toward,
        translate_up,
    },
};

pub(crate) trait ToInstances {
    fn to_instances(&self) -> Instances;
}

macro_rules! populate_all_faces {
    ($tr:ident, $co:ident, $sm:ident, $sl:ident) => {
        populate_all_faces!($tr, $co, $sm, $sl, Up, Down, Front, Right, Back, Left);
    };
    ($tr:ident, $co:ident, $sm:ident, $sl:ident, $($face:ident),*) => {
        $(
            let (new_transformations, new_colours) = face_to_instances(Face::$face, $sm, $sl);
            $tr.extend(new_transformations);
            $co.extend(new_colours);
        )*
    };
}

impl ToInstances for Cube {
    fn to_instances(&self) -> Instances {
        let side_length = self.side_length();
        let capacity = 6 * side_length * side_length;
        let mut transformations = Vec::with_capacity(capacity);
        let mut colours = Vec::with_capacity(capacity);

        let side_map = self.side_map();
        populate_all_faces!(transformations, colours, side_map, side_length);

        Instances {
            transformations,
            colors: Some(colours),
            ..Default::default()
        }
    }
}

fn face_to_instances(
    face: Face,
    side_map: &SideMap,
    side_length: usize,
) -> (
    impl Iterator<Item = Matrix4<f32>> + '_,
    impl Iterator<Item = Srgba> + '_,
) {
    let side = &side_map[face];

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

fn cubie_face_to_transformation(
    side_length: usize,
    face: Face,
    x: usize,
    y: usize,
) -> Matrix4<f32> {
    move_face_into_place(face)
        * position_from_origin_centered_to(side_length as f32, x as f32, y as f32)
        * scale_down(side_length as f32)
}

fn move_face_into_place(face: Face) -> Matrix4<f32> {
    match face {
        Face::Up => translate_up() * rev_quarter_turn_around_x(),
        Face::Down => translate_down() * quarter_turn_around_x(),
        Face::Front => translate_toward(),
        Face::Right => translate_right() * quarter_turn_around_y(),
        Face::Back => translate_away() * half_turn_around_y(),
        Face::Left => translate_left() * rev_quarter_turn_around_y(),
    }
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
