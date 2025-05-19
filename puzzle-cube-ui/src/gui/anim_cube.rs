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
        progress_linear: f32,
        seq: Option<Box<dyn Iterator<Item = Rotation>>>,
    },
    TransitioningToNext {
        rotation: Rotation,
        seq: Option<Box<dyn Iterator<Item = Rotation>>>,
    },
}

impl AnimationState {
    fn is_animating(&self) -> bool {
        !matches!(self, AnimationState::Stationary)
    }

    fn progress_animation(&mut self, elapsed_time: f64) {
        match self {
            AnimationState::Stationary => {}
            AnimationState::Rotating {
                progress_linear,
                seq,
                ..
            } if *progress_linear >= 1. => {
                let seq = seq.take();
                if let Some(mut iter) = seq {
                    if let Some(next_rot) = iter.next() {
                        debug!(
                            "progress_animation setting anim state to TransitioningToNext {next_rot:?} and some iter"
                        );
                        *self = AnimationState::TransitioningToNext {
                            rotation: next_rot,
                            seq: Some(iter),
                        };
                        return;
                    }
                }
                debug!("progress_animation setting anim state to Stationary");
                *self = AnimationState::Stationary;
            }
            AnimationState::Rotating {
                rotation,
                progress_linear,
                seq,
            } => {
                #[expect(clippy::cast_possible_truncation)]
                let new_progress = *progress_linear + (elapsed_time as f32 * ANIM_SPEED);
                let new_progress = new_progress.clamp(0., 1.);
                debug!("progress_animation calculated new progress {new_progress}");
                *self = AnimationState::Rotating {
                    rotation: *rotation,
                    progress_linear: new_progress,
                    seq: seq.take(),
                };
            }
            AnimationState::TransitioningToNext { rotation, seq } => {
                debug!(
                    "progress_animation setting anim state from TransitioningToNext to Rotating at 0 progress"
                );
                *self = AnimationState::Rotating {
                    rotation: *rotation,
                    progress_linear: 0.,
                    seq: seq.take(),
                };
            }
        }
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
        if let AnimationState::TransitioningToNext { rotation, .. } = self.animation {
            self.cube
                .rotate(rotation)
                .expect("ui only allows valid rotation sequences");
        }
        self.animation.progress_animation(elapsed_time);
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
            progress_linear: 0.,
            seq: None,
        };
        self.cube.rotate(rotation)
    }

    fn rotate_seq(
        &mut self,
        rotations: impl IntoIterator<Item = Rotation> + 'static,
    ) -> anyhow::Result<()> {
        let mut rotations = rotations.into_iter();
        if let Some(rotation) = rotations.next() {
            self.animation = AnimationState::TransitioningToNext {
                rotation,
                seq: Some(Box::new(rotations)),
            };
        }
        Ok(())
    }
}

impl<C: PuzzleCube + Display> Display for AnimCube<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cube)
    }
}
