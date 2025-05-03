use std::fmt::Display;

use rusty_puzzle_cube::cube::{PuzzleCube, rotation::Rotation, side_lengths::SideLength};

use super::cube_ext::AsInstances;

pub(crate) struct AnimCube<C: PuzzleCube + AsInstances> {
    cube: C,
    #[expect(unused)]
    animation: AnimationState,
}

#[derive(Default)]
enum AnimationState {
    #[default]
    Stationary,
    #[expect(unused)]
    Rotating {
        rotation: Rotation, // todo or a before and after? tbc
        progress: f32,
    },
}

impl<C: PuzzleCube + AsInstances> AnimCube<C> {
    pub fn new(cube: C) -> Self {
        AnimCube {
            cube,
            animation: AnimationState::default(),
        }
    }
}

impl<C: PuzzleCube + AsInstances> PuzzleCube for AnimCube<C> {
    fn recreate_at_size(&self, side_length: SideLength) -> Self {
        let cube = self.cube.recreate_at_size(side_length);
        AnimCube {
            cube,
            animation: AnimationState::Stationary,
        }
    }

    fn side_length(&self) -> usize {
        self.cube.side_length()
    }

    fn side_map(&self) -> &rusty_puzzle_cube::cube::SideMap {
        self.cube.side_map()
    }

    fn rotate(&mut self, rotation: Rotation) -> anyhow::Result<()> {
        let _ = self.cube.rotate(rotation);
        // todo lerp between some before and after positions (maybe capture a before and after instances vec?)
        Ok(())
    }
}

impl<C: PuzzleCube + AsInstances> AsInstances for AnimCube<C> {
    fn as_instances(&self) -> three_d::Instances {
        self.cube.as_instances()
    }
}

impl<C: PuzzleCube + AsInstances + Display> Display for AnimCube<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cube)
    }
}
