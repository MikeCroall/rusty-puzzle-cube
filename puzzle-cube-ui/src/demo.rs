use crate::gui::start_gui;

use std::time::Instant;

use rusty_puzzle_cube::{
    cube::{Cube, PuzzleCube as _, face::Face, rotation::Rotation},
    known_transforms::{checkerboard_corners, cube_in_cube_in_cube},
};
use tracing::error;

pub fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    if let Err(e) = start_gui() {
        error!("Could not start gui, defaulting to terminal demo: {}", e);
        terminal_demos().expect("demos are known to be valid");
    }
}

fn terminal_demos() -> anyhow::Result<()> {
    demo_simple_turns()?;
    demo_simple_turns_big_cube()?;
    demo_checkerboard()?;
    demo_cube_in_cube_in_cube()?;
    demo_inner_rotation()?;
    demo_inner_rotation_recreate_checkerboard()?;
    demo_simple_inner_rotation_medium_cube()?;
    demo_inner_rotation_large_cube()?;
    demo_shuffle()?;
    Ok(())
}

fn demo_simple_turns() -> anyhow::Result<()> {
    println!("Demo of simple turns and their inverse");

    let start_time = Instant::now();

    let mut cube = Cube::default();
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Front))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Right))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Back))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Back))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Right))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Front))?;
    println!();
    print!("{cube}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
    Ok(())
}

fn demo_simple_turns_big_cube() -> anyhow::Result<()> {
    println!("Demo of simple turns and their inverse on a big cube");

    let start_time = Instant::now();

    let mut cube = Cube::create_with_unique_characters(8.try_into()?);
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Front))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Right))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::clockwise(Face::Back))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Back))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Right))?;
    println!();
    print!("{cube}");
    cube.rotate(Rotation::anticlockwise(Face::Front))?;
    println!();
    print!("{cube}");

    let elapsed = start_time.elapsed();
    println!("Overall (printing included) this demo took {elapsed:?}\n");
    Ok(())
}

macro_rules! demo_timing {
    ($title:literal, $closure:tt) => {
        demo_timing!($title, (|| Cube::default()), $closure)
    };
    ($title:literal, $create_cube_closure:tt, $perform_moves_closure:tt) => {{
        println!($title);

        let start_time = Instant::now();

        let mut cube = $create_cube_closure();
        println!("Cube before:\n{cube}");

        let start_time_transform_only = Instant::now();
        $perform_moves_closure(&mut cube)?;
        let elapsed_transform_only = start_time_transform_only.elapsed();

        println!("Cube after:\n{cube}");

        let elapsed = start_time.elapsed();
        println!("Overall (printing included) this demo took {elapsed:?} (transform only took {elapsed_transform_only:?})\n");
        Ok(())
    }};
}

fn demo_checkerboard() -> anyhow::Result<()> {
    demo_timing!(
        "Demo of checkerboard pattern",
        (|cube| -> anyhow::Result<()> {
            checkerboard_corners(cube);
            Ok(())
        })
    )
}

fn demo_cube_in_cube_in_cube() -> anyhow::Result<()> {
    demo_timing!(
        "Demo of cube in cube in cube",
        (|cube| -> anyhow::Result<()> {
            cube_in_cube_in_cube(cube);
            Ok(())
        })
    )
}

fn demo_inner_rotation() -> anyhow::Result<()> {
    demo_timing!(
        "Demo of rotating inner slice",
        (|cube: &mut Cube| -> anyhow::Result<()> {
            cube.rotate(Rotation::clockwise_setback_from(Face::Front, 1))?;
            cube.rotate(Rotation::clockwise_setback_from(Face::Up, 1))?;
            Ok(())
        })
    )
}

fn demo_inner_rotation_recreate_checkerboard() -> anyhow::Result<()> {
    demo_timing!(
        "Demo of rotating inner slice",
        (|cube: &mut Cube| -> anyhow::Result<()> {
            let axis1 = Rotation::clockwise_setback_from(Face::Front, 1);
            cube.rotate(axis1)?;
            cube.rotate(axis1)?;

            let axis2 = Rotation::clockwise_setback_from(Face::Right, 1);
            cube.rotate(axis2)?;
            cube.rotate(axis2)?;

            let axis3 = Rotation::clockwise_setback_from(Face::Up, 1);
            cube.rotate(axis3)?;
            cube.rotate(axis3)?;
            Ok(())
        })
    )
}

fn demo_simple_inner_rotation_medium_cube() -> anyhow::Result<()> {
    let side_length = 5;
    demo_timing!(
        "Demo of rotating inner slice on large cube",
        (|| Cube::create_with_unique_characters(
            side_length
                .try_into()
                .expect("known valid side length for unique chars")
        )),
        (|cube: &mut Cube| -> anyhow::Result<()> {
            cube.rotate(Rotation::clockwise_setback_from(Face::Front, 1))?;
            cube.rotate(Rotation::anticlockwise_setback_from(Face::Front, 3))?;
            cube.rotate(Rotation::anticlockwise_setback_from(Face::Right, 2))?;
            Ok(())
        })
    )
}

fn demo_inner_rotation_large_cube() -> anyhow::Result<()> {
    let side_length = 9;
    demo_timing!(
        "Demo of rotating inner slice on large cube",
        (|| Cube::create(
            side_length
                .try_into()
                .expect("known valid side length if not unique chars")
        )),
        (|cube: &mut Cube| -> anyhow::Result<()> {
            for layer in (0..side_length).step_by(2) {
                cube.rotate(Rotation::clockwise_setback_from(Face::Front, layer))?;
            }
            for layer in (0..side_length).step_by(2) {
                cube.rotate(Rotation::clockwise_setback_from(Face::Right, layer))?;
            }
            for layer in (0..side_length).step_by(2) {
                cube.rotate(Rotation::clockwise_setback_from(Face::Up, layer))?;
            }
            for layer in (0..side_length).step_by(2) {
                cube.rotate(Rotation::clockwise_setback_from(Face::Front, layer))?;
            }
            Ok(())
        })
    )
}

fn demo_shuffle() -> anyhow::Result<()> {
    demo_timing!(
        "Demo of 100-move shuffling a 9x9x9 cube",
        (|| Cube::create(
            9.try_into()
                .expect("known valid side length if not unique chars")
        )),
        (|cube: &mut Cube| -> anyhow::Result<()> {
            cube.shuffle(100);
            Ok(())
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_demos_are_valid() -> anyhow::Result<()> {
        terminal_demos()
    }
}
