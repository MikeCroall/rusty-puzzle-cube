use crate::gui::cube_ext::ToInstances;
use rusty_puzzle_cube::cube::{face::Face, Cube};
use three_d::{
    egui::{epaint, special_emojis::GITHUB, Checkbox, FontId, Slider, TextStyle, Ui},
    Camera, ColorMaterial, Context, Gm, InstancedMesh, Mesh, Viewport,
};
use tracing::{error, info};

use super::defaults::initial_camera;
#[cfg(not(target_arch = "wasm32"))]
use super::file_io::save_as_image;

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const UNREASONABLE_MAX_CUBE_SIZE: usize = 2000;

macro_rules! rotate_buttons {
    ($ui:ident, $cube:ident, $instanced_square:ident) => {
        rotate_buttons!($ui, $cube, $instanced_square, "F", Front);
        rotate_buttons!($ui, $cube, $instanced_square, "R", Right);
        rotate_buttons!($ui, $cube, $instanced_square, "U", Up);
        rotate_buttons!($ui, $cube, $instanced_square, "L", Left);
        rotate_buttons!($ui, $cube, $instanced_square, "B", Back);
        rotate_buttons!($ui, $cube, $instanced_square, "D", Down);
    };
    ($ui:ident, $cube:ident, $instanced_square:ident, $text:literal, $face:ident) => {
        $ui.horizontal(|ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Button,
                FontId::new(24.0, epaint::FontFamily::Proportional),
            );
            if ui.button($text).clicked() {
                $cube.rotate_face_90_degrees_clockwise(Face::$face);
                $instanced_square.set_instances(&$cube.to_instances());
            }
            if ui.button(format!("{}'", $text)).clicked() {
                $cube.rotate_face_90_degrees_anticlockwise(Face::$face);
                $instanced_square.set_instances(&$cube.to_instances());
            }
        });
    };
}

pub(super) fn header(ui: &mut Ui) {
    ui.heading("Rusty Puzzle Cube");
    ui.label("By Mike Croall");
    ui.hyperlink_to(
        format!("{GITHUB} on GitHub"),
        "https://github.com/MikeCroall/rusty-puzzle-cube/",
    );
    ui.separator();
}

pub(super) fn initialise_cube(
    ui: &mut Ui,
    unreasonable_mode: &mut bool,
    side_length: &mut usize,
    cube: &mut Cube,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.heading("Initialise Cube");
    let slider_max_value = if *unreasonable_mode {
        UNREASONABLE_MAX_CUBE_SIZE
    } else {
        MAX_CUBE_SIZE
    };
    let prev_side_length = *side_length;
    ui.add(
        Slider::new(side_length, MIN_CUBE_SIZE..=slider_max_value)
            .text(format!("{prev_side_length}x{prev_side_length} Cube")),
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
        *cube = Cube::create(*side_length);
        instanced_square.set_instances(&cube.to_instances());
    }
    ui.separator();
}

pub(super) fn control_cube(
    ui: &mut Ui,
    cube: &mut Cube,
    instanced_square: &mut Gm<InstancedMesh, ColorMaterial>,
) {
    ui.heading("Control Cube");
    rotate_buttons!(ui, cube, instanced_square);
    ui.label("Moves that don't also apply to 3x3 cubes are not currently supported");
    ui.separator();
}

pub(super) fn control_camera(
    ui: &mut Ui,
    camera: &mut Camera,
    viewport: Viewport,
    render_axes: &mut bool,
) {
    ui.heading("Control Camera etc.");
    if ui.button("Reset camera").clicked() {
        *camera = initial_camera(viewport);
    }
    ui.add(Checkbox::new(render_axes, "Show axes"));
    ui.label("F is the blue axis\nR is the red axis\nU is the green axis");
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
