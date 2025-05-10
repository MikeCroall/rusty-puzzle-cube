use rusty_puzzle_cube::cube::{PuzzleCube, SideMap, rotation::Rotation, side_lengths::SideLength};
use std::fmt::Display;
use tracing::debug;

const ANIM_SPEED: f32 = 0.005;

pub(crate) struct AnimCube<C: PuzzleCube> {
    cube: C,
    pub(crate) animation: AnimationState,
}

#[derive(Default)]
pub(crate) enum AnimationState {
    #[default]
    Stationary,
    Rotating {
        rotation: Rotation,
        progress: f32,
    },
}

impl AnimationState {
    fn is_animating(&self) -> bool {
        !matches!(self, AnimationState::Stationary)
    }
}

impl<C: PuzzleCube> AnimCube<C> {
    pub fn new(cube: C) -> Self {
        AnimCube {
            cube,
            animation: AnimationState::default(),
        }
    }

    pub fn is_animating(&self) -> bool {
        self.animation.is_animating()
    }

    pub fn progress_animation(&mut self, elapsed_time: f64) {
        match self.animation {
            AnimationState::Stationary => {}
            AnimationState::Rotating { progress, .. } if progress >= 1. => {
                self.animation = AnimationState::Stationary;
            }
            AnimationState::Rotating { rotation, progress } => {
                #[expect(clippy::cast_possible_truncation)]
                let new_progress = progress + (elapsed_time as f32 * ANIM_SPEED);
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

impl<C: PuzzleCube> PuzzleCube for AnimCube<C> {
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

    fn side_map(&self) -> &SideMap {
        self.cube.side_map()
    }

    fn rotate(&mut self, rotation: Rotation) -> anyhow::Result<()> {
        let rotation = rotation.normalise(self.side_length());
        self.animation = AnimationState::Rotating {
            rotation,
            progress: 0.,
        };
        self.cube.rotate(rotation)
    }
}

impl<C: PuzzleCube + Display> Display for AnimCube<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cube)
    }
}
