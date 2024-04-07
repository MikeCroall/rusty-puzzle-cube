use std::time::Instant;

use crate::{
    cube::{face::Face, Cube},
    known_transforms::{checkerboard_corners, cube_in_cube_in_cube},
};

pub mod cube;
pub mod known_transforms;
pub mod notation;

fn main() {
    demo_simple_turns();
    demo_simple_turns_big_cube();
    demo_checkerboard();
    demo_cube_in_cube_in_cube();
}

fn demo_simple_turns() {
    println!("Demo of simple turns and their inverse");

    let start_time = Instant::now();

    let mut cube = Cube::create(3);
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    print!("{cube:?}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
}

fn demo_simple_turns_big_cube() {
    println!("Demo of simple turns and their inverse on a big cube");

    let start_time = Instant::now();

    let mut cube = Cube::create_with_unique_characters(8);
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    print!("{cube:?}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    print!("{cube:?}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
}

fn demo_checkerboard() {
    println!("Demo of checkerboard pattern");

    let start_time = Instant::now();

    let mut cube = Cube::create(3);
    println!("Cube before:\n{cube:?}");

    let start_time_transform_only = Instant::now();
    checkerboard_corners(&mut cube);
    let elapsed_transform_only = start_time_transform_only.elapsed();

    println!("Cube after:\n{cube:?}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?} (transform only took {elapsed_transform_only:?})\n");
}

fn demo_cube_in_cube_in_cube() {
    println!("Demo of cube in cube in cube");

    let start_time = Instant::now();

    let mut cube = Cube::create(3);
    println!("Cube before:\n{cube:?}");

    let start_time_transform_only = Instant::now();
    cube_in_cube_in_cube(&mut cube);
    let elapsed_transform_only = start_time_transform_only.elapsed();

    println!("Cube after:\n{cube:?}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?} (transform only took {elapsed_transform_only:?})\n");
}
