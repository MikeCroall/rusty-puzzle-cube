use rusty_puzzle_cube::cube::{
    DefaultSide, PuzzleCube, face::Face, rotation::Rotation, side_lengths::SideLength,
};
use std::fmt::Display;
use tracing::debug;

const ANIM_SPEED: f32 = 0.005;

pub(crate) struct AnimCube<C: PuzzleCube<Side = DefaultSide>> {
    cube: C,
    pub(crate) animation: AnimationState,
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct AnimationProgress {
    pub(crate) sequence_total: Option<usize>,
    pub(crate) sequence_current: usize,
    pub(crate) single_rotation_linear: f32,
}

impl AnimationProgress {
    #[expect(clippy::cast_precision_loss)]
    pub(crate) fn sequence_linear_with_sub_step(&self) -> Option<f32> {
        self.sequence_total.map(|total| total as f32).map(|total| {
            let sub_step_adjustment = -(1. - self.single_rotation_linear) / total;
            (self.sequence_current as f32 / total) + sub_step_adjustment
        })
    }
}

#[derive(Default)]
pub(crate) enum AnimationState {
    #[default]
    Stationary,
    Rotating {
        rotation: Rotation,
        progress: AnimationProgress,
        seq: Option<Box<dyn Iterator<Item = Rotation>>>,
    },
    TransitioningToNext {
        rotation: Rotation,
        progress: AnimationProgress,
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
            AnimationState::Rotating { progress, seq, .. }
                if progress.single_rotation_linear >= 1. =>
            {
                let seq = seq.take();
                if let Some(mut iter) = seq {
                    if let Some(next_rot) = iter.next() {
                        debug!(
                            "progress_animation setting anim state to TransitioningToNext {next_rot:?} and some iter"
                        );
                        *self = AnimationState::TransitioningToNext {
                            rotation: next_rot,
                            progress: *progress,
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
                progress,
                seq,
            } => {
                #[expect(clippy::cast_possible_truncation)]
                let new_progress =
                    progress.single_rotation_linear + (elapsed_time as f32 * ANIM_SPEED);
                let new_progress = new_progress.clamp(0., 1.);
                debug!("progress_animation calculated new progress {new_progress}");
                *self = AnimationState::Rotating {
                    rotation: *rotation,
                    progress: AnimationProgress {
                        single_rotation_linear: new_progress,
                        ..*progress
                    },
                    seq: seq.take(),
                };
            }
            AnimationState::TransitioningToNext {
                rotation,
                progress,
                seq,
            } => {
                debug!(
                    "progress_animation setting anim state from TransitioningToNext to Rotating at 0 progress"
                );
                *self = AnimationState::Rotating {
                    rotation: *rotation,
                    progress: AnimationProgress {
                        single_rotation_linear: 0.,
                        sequence_current: progress.sequence_current + 1,
                        ..*progress
                    },
                    seq: seq.take(),
                };
            }
        }
    }
}

impl<C: PuzzleCube<Side = DefaultSide>> AnimCube<C> {
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

impl<C: PuzzleCube<Side = DefaultSide>> PuzzleCube for AnimCube<C> {
    type Side = C::Side;

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

    fn side(&self, face: Face) -> &Self::Side {
        self.cube.side(face)
    }

    fn rotate(&mut self, rotation: Rotation) -> anyhow::Result<()> {
        let rotation = rotation.normalise(self.side_length());
        self.animation = AnimationState::Rotating {
            rotation,
            progress: AnimationProgress::default(),
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
                progress: AnimationProgress::default(),
                seq: Some(Box::new(rotations)),
            };
        }
        Ok(())
    }
}

impl<C: PuzzleCube<Side = DefaultSide> + Display> Display for AnimCube<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cube)
    }
}
