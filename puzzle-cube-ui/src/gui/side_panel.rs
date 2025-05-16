use std::fmt::Display;

use circular_buffer::CircularBuffer;
use rusty_puzzle_cube::cube::{PuzzleCube, rotation::Rotation, side_lengths::SideLength};
use three_d::{
    Camera, ColorMaterial, Gm, InstancedMesh, Mesh, Viewport,
    egui::{Button, Checkbox, Rgba, ScrollArea, SidePanel, Slider, Ui, special_emojis::GITHUB},
};
use tracing::{error, info};

#[cfg(not(target_arch = "wasm32"))]
use super::file_io::save_as_image;
use super::{cube_3d_ext::PuzzleCube3D, defaults::initial_camera};

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const EXTRA_SPACING: f32 = 10.;

#[expect(clippy::too_many_arguments)]
pub(super) fn draw_side_panel<C: PuzzleCube3D + Display, const UNDO_SIZE: usize>(
    side_length: &mut usize,
    cube: &mut C,
    undo_queue: &mut CircularBuffer<UNDO_SIZE, Rotation>,
    camera: &mut Camera,
    lock_upright: &mut bool,
    ctx: &three_d::Context,
    tiles: &mut Gm<InstancedMesh, ColorMaterial>,
    pick_cube: &Gm<Mesh, ColorMaterial>,
    render_axes: &mut bool,
    viewport: Viewport,
    gui_ctx: &three_d::egui::Context,
) {
    SidePanel::left("side_panel").show(gui_ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            header(ui);
            ui.separator();

            initialise_cube(ui, side_length, cube, undo_queue, tiles);
            ui.separator();

            control_cube(ui, cube, undo_queue, tiles);
            ui.separator();

            control_camera(ui, camera, lock_upright, viewport, render_axes);
            ui.separator();

            #[cfg(not(target_arch = "wasm32"))]
            debug_ctrls(ui, &*cube, ctx, viewport, &*camera, &*tiles, pick_cube);
        })
    });
}

fn header(ui: &mut Ui) {
    ui.heading("Rusty Puzzle Cube");
    ui.label("By Mike Croall");
    ui.hyperlink_to(
        format!("{GITHUB} on GitHub"),
        "https://github.com/MikeCroall/rusty-puzzle-cube/",
    );
    ui.add_space(EXTRA_SPACING);
}

fn initialise_cube<C: PuzzleCube3D, const UNDO_SIZE: usize>(
    ui: &mut Ui,
    side_length: &mut usize,
    cube: &mut C,
    undo_queue: &mut CircularBuffer<UNDO_SIZE, Rotation>,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Initialise Cube");
    ui.add_space(EXTRA_SPACING);

    let prev_side_length = *side_length;
    ui.add(Slider::new(side_length, MIN_CUBE_SIZE..=MAX_CUBE_SIZE));
    ui.add_space(EXTRA_SPACING);

    if ui
        .button(format!(
            "New {prev_side_length}x{prev_side_length}x{prev_side_length} Cube"
        ))
        .clicked()
    {
        let side_length = SideLength::try_from(*side_length)
            .expect("UI is configured to only allow selecting valid side length values");
        *cube = cube.recreate_at_size(side_length);
        undo_queue.clear();
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.add_space(EXTRA_SPACING);
}

fn control_cube<C: PuzzleCube3D, const UNDO_SIZE: usize>(
    ui: &mut Ui,
    cube: &mut C,
    undo_queue: &mut CircularBuffer<UNDO_SIZE, Rotation>,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Control Cube");
    ui.label("Click and drag directly on the cube to make a rotation");
    ui.label("You must only drag across one face of the cube");
    ui.label(
        "Dragging to another face, diagonally, or for a very small distance will be cancelled",
    );
    ui.add_space(EXTRA_SPACING);

    let undo_text = if undo_queue.is_full() {
        format!("Undo ({}, at limit)", undo_queue.len())
    } else if !undo_queue.is_empty() {
        format!("Undo ({})", undo_queue.len())
    } else {
        "Undo".to_owned()
    };
    if ui
        .add_enabled(!undo_queue.is_empty(), Button::new(undo_text))
        .clicked()
    {
        let to_undo = undo_queue
            .pop_back()
            .expect("button disabled if queue empty");
        cube.rotate(!to_undo)
            .expect("moves on queue must be reversible");
    }
    ui.add_space(EXTRA_SPACING);

    let shuffle_moves = cube.side_length() * 10;
    if ui
        .button(format!("Shuffle ({shuffle_moves} moves)"))
        .clicked()
    {
        cube.shuffle(shuffle_moves);
        cube.cancel_animation();
        undo_queue.clear();
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.add_space(EXTRA_SPACING);
}

fn control_camera(
    ui: &mut Ui,
    camera: &mut Camera,
    lock_upright: &mut bool,
    viewport: Viewport,
    render_axes: &mut bool,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Control Camera etc.");
    ui.label("The camera can be moved with a click and drag starting from the blank space around the cube, or by dragging from one face to any other face or empty space");
    ui.add_space(EXTRA_SPACING);

    if ui.button("Reset camera").clicked() {
        *camera = initial_camera(viewport);
    }
    ui.add_space(EXTRA_SPACING);

    if ui
        .add(Checkbox::new(lock_upright, "Lock upright"))
        .changed()
        && *lock_upright
    {
        *camera = initial_camera(viewport);
    }
    ui.add_space(EXTRA_SPACING);

    ui.add(Checkbox::new(render_axes, "Show axes"));
    if *render_axes {
        ui.colored_label(Rgba::from_rgb(0.15, 0.15, 1.), "F is the blue axis");
        ui.colored_label(Rgba::RED, "R is the red axis");
        ui.colored_label(Rgba::GREEN, "U is the green axis");
    }
    ui.add_space(EXTRA_SPACING);
}

#[cfg(not(target_arch = "wasm32"))]
fn debug_ctrls<C: PuzzleCube + Display>(
    ui: &mut Ui,
    cube: &C,
    ctx: &three_d::Context,
    viewport: Viewport,
    camera: &Camera,
    tiles: &Gm<InstancedMesh, ColorMaterial>,
    inner_cube: &Gm<Mesh, ColorMaterial>,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Debug");
    ui.add_space(EXTRA_SPACING);

    if ui.button("Print cube to terminal").clicked() {
        info!("\n{cube}");
    }
    ui.add_space(EXTRA_SPACING);

    if ui.button("Save as image").clicked() {
        if let Err(e) = save_as_image(ctx, viewport, camera, tiles, inner_cube) {
            error!("Could not save image file: {}", e);
        }
    }
    ui.add_space(EXTRA_SPACING);
}
