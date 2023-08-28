use std::time::Instant;

use crate::cube::{face::Face, Cube};

mod cube;

fn main() {
    let side_length = 3;

    println!("Trying {0}x{0}x{0} cube", side_length);
    let start_time = Instant::now();

    let mut cube = Cube::create(side_length);
    cube.print_cube();
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    cube.print_cube();

    let elapsed = start_time.elapsed();
    println!("Overall this cube took {:?}", elapsed);
}
