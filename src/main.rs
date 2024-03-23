use std::time::Instant;

use crate::cube::{face::Face, Cube};

mod cube;

fn main() {
    let side_length = 3;

    println!("Trying {side_length}x{side_length}x{side_length} cube");
    let start_time = Instant::now();

    let mut cube = Cube::create(side_length);
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    cube.print_cube();
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    cube.print_cube();
    let elapsed = start_time.elapsed();
    println!("Overall this cube took {elapsed:?}");
}
