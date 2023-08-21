use std::time::Instant;

use crate::cube::Cube;

mod cube;

fn main() {
    for side_length in 0..8 {
        println!("Trying {0}x{0}x{0} cube", side_length);
        let start_time = Instant::now();

        let cube = Cube::create(side_length);
        cube.print_cube();

        let elapsed = start_time.elapsed();
        println!(
            "Creating and printing {0}x{0}x{0} cube took {1:?}\n",
            side_length, elapsed
        );
    }
}
