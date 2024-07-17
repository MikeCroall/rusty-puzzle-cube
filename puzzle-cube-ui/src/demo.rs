use crate::gui::start_gui;

use std::time::Instant;

use rusty_puzzle_cube::{
    cube::{cube_slice::CubeSliceTwist, face::Face, Cube},
    known_transforms::{checkerboard_corners, cube_in_cube_in_cube},
};
use tracing::error;

pub fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    if let Err(e) = start_gui() {
        error!("Could not start gui, defaulting to terminal demo: {}", e);
        demo_simple_turns();
        demo_simple_turns_big_cube();
        demo_checkerboard();
        demo_cube_in_cube_in_cube();
        demo_inner_rotation();
        demo_inner_rotation_recreate_checkerboard();
    }
}

fn demo_simple_turns() {
    println!("Demo of simple turns and their inverse");

    let start_time = Instant::now();

    let mut cube = Cube::default();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    print!("{cube}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
}

fn demo_simple_turns_big_cube() {
    println!("Demo of simple turns and their inverse on a big cube");

    let start_time = Instant::now();

    let mut cube = Cube::default();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Front);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Right);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_clockwise(Face::Back);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Back);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Right);
    println!();
    print!("{cube}");
    cube.rotate_face_90_degrees_anticlockwise(Face::Front);
    println!();
    print!("{cube}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
}

macro_rules! demo_timing {
    ($title:literal, $closure:tt) => {
        println!($title);

        let start_time = Instant::now();

        let mut cube = Cube::default();
        println!("Cube before:\n{cube}");

        let start_time_transform_only = Instant::now();
        $closure(&mut cube);
        let elapsed_transform_only = start_time_transform_only.elapsed();

        println!("Cube after:\n{cube}");

        let elapsed = start_time.elapsed();
        println!("Overall (printing included) this demo took {elapsed:?} (transform only took {elapsed_transform_only:?})\n");
    };
}

fn demo_checkerboard() {
    demo_timing!(
        "Demo of checkerboard pattern",
        (|cube| { checkerboard_corners(cube) })
    );
}

fn demo_cube_in_cube_in_cube() {
    demo_timing!(
        "Demo of cube in cube in cube",
        (|cube| { cube_in_cube_in_cube(cube) })
    );
}

fn demo_inner_rotation() {
    demo_timing!(
        "Demo of rotating inner slice",
        (|cube: &mut Cube| {
            cube.rotate_inner_slice(CubeSliceTwist {
                relative_to: Face::Front,
                layer: 2,
                clockwise: true,
            })
            .expect("Demo is known to be valid");
            cube.rotate_inner_slice(CubeSliceTwist {
                relative_to: Face::Up,
                layer: 2,
                clockwise: true,
            })
            .expect("Demo is known to be valid");
        })
    );
}

fn demo_inner_rotation_recreate_checkerboard() {
    demo_timing!(
        "Demo of rotating inner slice",
        (|cube: &mut Cube| {
            let axis1 = CubeSliceTwist {
                relative_to: Face::Front,
                layer: 2,
                clockwise: true,
            };
            cube.rotate_inner_slice(axis1)
                .expect("Demo is known to be valid");
            cube.rotate_inner_slice(axis1)
                .expect("Demo is known to be valid");

            let axis2 = CubeSliceTwist {
                relative_to: Face::Right,
                layer: 2,
                clockwise: true,
            };
            cube.rotate_inner_slice(axis2)
                .expect("Demo is known to be valid");
            cube.rotate_inner_slice(axis2)
                .expect("Demo is known to be valid");

            let axis3 = CubeSliceTwist {
                relative_to: Face::Up,
                layer: 2,
                clockwise: true,
            };
            cube.rotate_inner_slice(axis3)
                .expect("Demo is known to be valid");
            cube.rotate_inner_slice(axis3)
                .expect("Demo is known to be valid");
        })
    );
    todo!("This is showing that not every angle works yet!");
}
