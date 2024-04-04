use std::time::Instant;

use crate::{
    cube::{face::Face, Cube},
    known_transforms_3x3::{checkerboard_corners, cube_in_cube_in_cube},
};

mod cube;
mod known_transforms_3x3;
mod notation;

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
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    cube.print_cube();

    let elapsed = start_time.elapsed();
    println!("Overall this demo took {elapsed:?}\n");
}

fn demo_simple_turns_big_cube() {
    println!("Demo of simple turns and their inverse on a big cube");

    let start_time = Instant::now();

    let mut cube = Cube::create_with_unique_characters(8);
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    cube.print_cube();

    let elapsed = start_time.elapsed();
    println!("Overall this demo took {elapsed:?}\n");
}

fn demo_checkerboard() {
    println!("Demo of checkerboard pattern");

    let start_time = Instant::now();

    let mut cube = Cube::create(3);
    cube.print_cube();
    checkerboard_corners(&mut cube);
    println!();
    cube.print_cube();

    let elapsed = start_time.elapsed();
    println!("Overall this demo took {elapsed:?}\n");
}

fn demo_cube_in_cube_in_cube() {
    println!("Demo of cube in cube in cube");

    let start_time = Instant::now();

    let mut cube = Cube::create(3);
    cube.print_cube();
    cube_in_cube_in_cube(&mut cube);
    println!();
    cube.print_cube();

    let elapsed = start_time.elapsed();
    println!("Overall this demo took {elapsed:?}\n");
}
