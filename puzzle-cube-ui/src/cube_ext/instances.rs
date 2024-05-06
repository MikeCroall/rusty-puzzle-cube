use rusty_puzzle_cube::cube::{cubie_face::CubieFace, face::Face, Cube, SideMap};
use three_d::{Instances, Matrix4, Srgba};

use crate::{
    colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
    combine_transforms,
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
            $tr.extend(new_transformations.iter());
            $co.extend(new_colours.iter());
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
) -> (Vec<Matrix4<f32>>, Vec<Srgba>) {
    let side = &side_map[face];
    let cubie_sides = side.iter().flatten().enumerate().map(|(i, cubie_face)| {
        let y = i / side_length;
        let x = i % side_length;
        (
            cubie_face_to_transformation(side_length, face, x, y),
            cubie_face_to_colour(cubie_face),
        )
    });
    cubie_sides.unzip()
}

fn cubie_face_to_transformation(
    side_length: usize,
    face: Face,
    _x: usize,
    _y: usize,
) -> Matrix4<f32> {
    if side_length != 1 {
        todo!("impl side_length,face,x,y to transformation - start for 1x1 cube (only needs face), then scale down and move around based on x,y to support 2x2+");
    }

    match face {
        Face::Up => combine_transforms!(quarter_turn_around_x, translate_up),
        Face::Down => combine_transforms!(quarter_turn_around_x, translate_down),
        Face::Front => combine_transforms!(translate_toward),
        Face::Right => combine_transforms!(quarter_turn_around_y, translate_right),
        Face::Back => combine_transforms!(translate_away),
        Face::Left => combine_transforms!(quarter_turn_around_y, translate_left),
    }
}

fn cubie_face_to_colour(cubie_face: &CubieFace) -> Srgba {
    match cubie_face {
        CubieFace::Blue(_) => BLUE,
        CubieFace::Green(_) => GREEN,
        CubieFace::Orange(_) => ORANGE,
        CubieFace::Red(_) => RED,
        CubieFace::White(_) => WHITE,
        CubieFace::Yellow(_) => YELLOW,
    }
}
