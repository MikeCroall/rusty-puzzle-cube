use std::fmt::Display;

use rusty_puzzle_cube::cube::{PuzzleCube, rotation::Rotation, side_lengths::SideLength};
use tracing::debug;

use super::cube_ext::PuzzleCube3D;

const ANIM_SPEED: f64 = 0.005;

pub(crate) struct AnimCube<C: PuzzleCube3D> {
    cube: C,
    animation: AnimationState,
}

#[derive(Default)]
enum AnimationState {
    #[default]
    Stationary,
    Rotating {
        rotation: Rotation,
        progress: f64,
    },
}

impl<C: PuzzleCube3D> AnimCube<C> {
    pub fn new(cube: C) -> Self {
        AnimCube {
            cube,
            animation: AnimationState::default(),
        }
    }

    pub fn is_animating(&self) -> bool {
        !matches!(self.animation, AnimationState::Stationary)
    }

    pub fn progress_animation(&mut self, elapsed_time: f64) {
        match self.animation {
            AnimationState::Stationary => {}
            AnimationState::Rotating { progress, .. } if progress >= 1. => {
                self.animation = AnimationState::Stationary;
            }
            AnimationState::Rotating { rotation, progress } => {
                let new_progress = progress + (elapsed_time * ANIM_SPEED);
                let new_progress = new_progress.clamp(0., 1.);
                debug!("progress_animation calculated new progress {new_progress}");
                self.animation = AnimationState::Rotating {
                    rotation,
                    progress: new_progress,
                }
            }
        }
    }
}

impl<C: PuzzleCube3D> PuzzleCube for AnimCube<C> {
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
        self.animation = AnimationState::Rotating {
            rotation,
            progress: 0.,
        };
        self.cube.rotate(rotation)
    }
}

impl<C: PuzzleCube3D> PuzzleCube3D for AnimCube<C> {
    fn as_instances(&self) -> three_d::Instances {
        // todo 'lerp' between some before and after positions but has to be rotate around origin, not linear a to b - should `rotate` save some before+after or other metadata into anim state to help?
        self.cube.as_instances()
    }
}

impl<C: PuzzleCube3D + Display> Display for AnimCube<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cube)
    }
}
