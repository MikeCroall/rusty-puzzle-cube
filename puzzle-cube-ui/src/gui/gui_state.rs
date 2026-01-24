use std::fmt::Display;

use crate::gui::{
    anim_cube::AnimCube, cube_3d_ext::PuzzleCube3D, defaults::initial_camera, initial_anim_cube,
    initial_instances, inner_cube,
};
use circular_buffer::CircularBuffer;
use rusty_puzzle_cube::{
    cube::{Cube, rotation::Rotation},
    known_transforms::KnownTransform,
};
use three_d::{Camera, ColorMaterial, Context, Gm, InstancedMesh, Window};
use tracing::info;

pub(crate) struct GuiState<C: PuzzleCube3D + Display, const UNDO_SIZE: usize> {
    pub(crate) side_length: usize,
    pub(crate) cube: C,
    pub(crate) undo_queue: CircularBuffer<UNDO_SIZE, Rotation>,
    pub(crate) selected_transform: KnownTransform,
    pub(crate) camera: Camera,
    pub(crate) lock_upright: bool,
    pub(crate) tiles: Gm<InstancedMesh, ColorMaterial>,
    pub(crate) render_axes: bool,
    pub(crate) animation_speed: f64,
    pub(crate) ctx: Context,
    pub(crate) pick_cube: Gm<three_d::Mesh, ColorMaterial>,
    pub(crate) rotation_if_released_now: Option<Rotation>,
}

impl<const UNDO_SIZE: usize> GuiState<AnimCube<Cube>, UNDO_SIZE> {
    pub(crate) fn init(window: &Window) -> anyhow::Result<Self> {
        info!("Initialising default cube");
        let side_length = 4;
        let cube = initial_anim_cube(side_length)?;
        let undo_queue = CircularBuffer::<UNDO_SIZE, Rotation>::new();

        info!("Initialising GUI");
        let ctx = window.gl();
        let camera = initial_camera(window.viewport());
        let pick_cube = inner_cube(&ctx);
        let tiles = initial_instances(&ctx, &cube);
        let rotation_if_released_now = None;

        Ok(Self {
            side_length,
            cube,
            undo_queue,
            selected_transform: KnownTransform::CheckerboardCorners3x3x3,
            camera,
            lock_upright: false,
            tiles,
            render_axes: false,
            animation_speed: 1.0,
            ctx,
            pick_cube,
            rotation_if_released_now,
        })
    }
}
