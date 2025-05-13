use std::fmt::Display;

use rusty_puzzle_cube::cube::{PuzzleCube, side_lengths::SideLength};
use three_d::{
    Camera, ColorMaterial, Gm, InstancedMesh, Mesh, Viewport,
    egui::{Checkbox, Rgba, ScrollArea, SidePanel, Slider, Ui, special_emojis::GITHUB},
};
use tracing::{error, info};

#[cfg(not(target_arch = "wasm32"))]
use super::file_io::save_as_image;
use super::{cube_3d_ext::PuzzleCube3D, defaults::initial_camera};

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const EXTRA_SPACING: f32 = 10.;

#[expect(clippy::too_many_arguments)]
pub(super) fn draw_side_panel<C: PuzzleCube3D + Display>(
    side_length: &mut usize,
    cube: &mut C,
    camera: &mut Camera,
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
            initialise_cube(ui, side_length, cube, tiles);
            control_cube(ui, cube, tiles);
            control_camera(ui, camera, viewport, render_axes);
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
    ui.separator();
}

fn initialise_cube<C: PuzzleCube3D>(
    ui: &mut Ui,
    side_length: &mut usize,
    cube: &mut C,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Initialise Cube");

    let prev_side_length = *side_length;
    ui.add(
        Slider::new(side_length, MIN_CUBE_SIZE..=MAX_CUBE_SIZE).text(format!(
            "{prev_side_length}x{prev_side_length}x{prev_side_length} Cube"
        )),
    );
    if ui.button("Apply").clicked() {
        let side_length = SideLength::try_from(*side_length)
            .expect("UI is configured to only allow selecting valid side length values");
        *cube = cube.recreate_at_size(side_length);
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.add_space(EXTRA_SPACING);
    ui.separator();
}

fn control_cube<C: PuzzleCube3D>(
    ui: &mut Ui,
    cube: &mut C,
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

    let shuffle_moves = cube.side_length() * 10;
    if ui
        .button(format!("Shuffle ({shuffle_moves} moves)"))
        .clicked()
    {
        cube.shuffle(shuffle_moves);
        cube.cancel_animation();
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.separator();
}

fn control_camera(ui: &mut Ui, camera: &mut Camera, viewport: Viewport, render_axes: &mut bool) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Control Camera etc.");
    ui.label("The camera can be moved with a click and drag starting from the blank space around the cube, or by dragging from one face to any other face or empty space");
    if ui.button("Reset camera").clicked() {
        *camera = initial_camera(viewport);
    }
    ui.add(Checkbox::new(render_axes, "Show axes"));
    if *render_axes {
        ui.colored_label(Rgba::from_rgb(0.15, 0.15, 1.), "F is the blue axis");
        ui.colored_label(Rgba::RED, "R is the red axis");
        ui.colored_label(Rgba::GREEN, "U is the green axis");
    }

    ui.add_space(EXTRA_SPACING);
    ui.separator();
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
    if ui.button("Print cube to terminal").clicked() {
        info!("\n{cube}");
    }

    if ui.button("Save as image").clicked() {
        if let Err(e) = save_as_image(ctx, viewport, camera, tiles, inner_cube) {
            error!("Could not save image file: {}", e);
        }
    }
}
