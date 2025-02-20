use rusty_puzzle_cube::cube::{Cube, side_lengths::SideLength};
use three_d::{
    Camera, ColorMaterial, Context, Gm, InstancedMesh, Mesh, Viewport,
    egui::{Checkbox, Rgba, Slider, Ui, special_emojis::GITHUB},
};
use tracing::{error, info};

#[cfg(not(target_arch = "wasm32"))]
use super::file_io::save_as_image;
use super::{cube_ext::AsInstances, defaults::initial_camera};

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const UNREASONABLE_MAX_CUBE_SIZE: usize = 2000;
const EXTRA_SPACING: f32 = 10.;

pub(super) fn header(ui: &mut Ui) {
    ui.heading("Rusty Puzzle Cube");
    ui.label("By Mike Croall");
    ui.hyperlink_to(
        format!("{GITHUB} on GitHub"),
        "https://github.com/MikeCroall/rusty-puzzle-cube/",
    );
    ui.add_space(EXTRA_SPACING);
    ui.separator();
}

pub(super) fn initialise_cube(
    ui: &mut Ui,
    unreasonable_mode: &mut bool,
    side_length: &mut usize,
    cube: &mut Cube,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.add_space(EXTRA_SPACING);
    ui.heading("Initialise Cube");
    let slider_max_value = if *unreasonable_mode {
        UNREASONABLE_MAX_CUBE_SIZE
    } else {
        MAX_CUBE_SIZE
    };
    let prev_side_length = *side_length;
    ui.add(
        Slider::new(side_length, MIN_CUBE_SIZE..=slider_max_value).text(format!(
            "{prev_side_length}x{prev_side_length}x{prev_side_length} Cube"
        )),
    );
    if ui
        .checkbox(unreasonable_mode, "Unreasonable mode")
        .changed()
        && !*unreasonable_mode
        && MAX_CUBE_SIZE < *side_length
    {
        *side_length = MAX_CUBE_SIZE;
    };
    if ui.button("Apply").clicked() {
        let side_length = SideLength::try_from(*side_length)
            .expect("UI is configured to only allow selecting valid side length values");
        *cube = Cube::create(side_length);
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.add_space(EXTRA_SPACING);
    ui.separator();
}

pub(super) fn control_cube(
    ui: &mut Ui,
    cube: &mut Cube,
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
        instanced_square.set_instances(&cube.as_instances());
    }
    ui.separator();
}

pub(super) fn control_camera(
    ui: &mut Ui,
    camera: &mut Camera,
    viewport: Viewport,
    render_axes: &mut bool,
) {
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
pub(super) fn debug(
    ui: &mut Ui,
    cube: &Cube,
    ctx: &Context,
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
