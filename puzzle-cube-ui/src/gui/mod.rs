mod colours;
mod cube_ext;
mod transforms;

use crate::gui::cube_ext::ToInstances;
use rusty_puzzle_cube::{
    cube::{face::Face, Cube},
    known_transforms::cube_in_cube_in_cube,
};
use three_d::{
    degrees,
    egui::{epaint, FontId, TextStyle},
    vec3, Axes, Camera, ClearState, ColorMaterial, Context, CpuMesh, FrameOutput, Gm,
    InstancedMesh, Mesh, Object, OrbitControl, Srgba, Viewport, Window, WindowSettings, GUI,
};
use tracing::{debug, error, info};

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const UNREASONABLE_MAX_CUBE_SIZE: usize = 2000;

pub(super) fn start_gui() -> Result<(), three_d::WindowError> {
    info!("Initialising default cube");
    let mut side_length = 3;
    let mut cube = Cube::create(side_length);
    cube_in_cube_in_cube(&mut cube);

    info!("Initialising GUI");
    let window = initial_window()?;
    let mut camera = initial_camera(window.viewport());
    let mut mouse_control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut unreasonable_mode = false;

    let ctx = window.gl();
    let mut gui = GUI::new(&ctx);
    let mut instanced_square = initial_instances(&ctx, &cube);

    // todo could make inner cube instances for each (external facing) cubie to make rotation animations less funky, when I actually add them...
    let inner_cube = inner_cube(&ctx);

    let mut render_axes = false;
    let axes = Axes::new(&ctx, 0.05, 2.);

    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;

        let mut panel_width = 0.;
        redraw |= gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_ctx| {
                use three_d::egui::{Checkbox, SidePanel, Slider, special_emojis::GITHUB};
                SidePanel::left("side_panel").show(gui_ctx, |ui| {
                    ui.heading("Rusty Puzzle Cube");
                    ui.label("By Mike Croall");
                    ui.hyperlink_to(format!("{GITHUB} on GitHub"), "https://github.com/MikeCroall/rusty-puzzle-cube/");
                    ui.separator();

                    ui.heading("Initialise Cube");
                    let slider_max_value = if unreasonable_mode {
                        UNREASONABLE_MAX_CUBE_SIZE
                    } else {
                        MAX_CUBE_SIZE
                    };
                    let prev_side_length = side_length;
                    ui.add(Slider::new(&mut side_length, MIN_CUBE_SIZE..=slider_max_value).text(format!("{prev_side_length}x{prev_side_length} Cube")));
                    if ui.checkbox(&mut unreasonable_mode, "Unreasonable mode").changed() && !unreasonable_mode && MAX_CUBE_SIZE < side_length {
                        side_length = MAX_CUBE_SIZE;
                    };
                    if ui.button("Apply").clicked() {
                        cube = Cube::create(side_length);
                        instanced_square.set_instances(&cube.to_instances());
                    }
                    ui.separator();

                    ui.heading("Control Cube");
                    macro_rules! rotate_buttons {
                        ($ui:ident, $cube:ident, $instanced_square:ident) => {
                            rotate_buttons!($ui, "F", $cube, Front, $instanced_square);
                            rotate_buttons!($ui, "R", $cube, Right, $instanced_square);
                            rotate_buttons!($ui, "U", $cube, Up, $instanced_square);
                            rotate_buttons!($ui, "L", $cube, Left, $instanced_square);
                            rotate_buttons!($ui, "B", $cube, Back, $instanced_square);
                            rotate_buttons!($ui, "D", $cube, Down, $instanced_square);
                        };
                        ($ui:ident, $text:literal, $cube:ident, $face:ident, $instanced_square:ident) => {
                            $ui.horizontal(|ui|{
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
                    rotate_buttons!(ui, cube, instanced_square);
                    ui.label("Moves that don't also apply to 3x3 cubes are not currently supported");
                    ui.separator();

                    ui.heading("Control Camera etc.");
                    if ui.button("Reset camera").clicked() {
                        camera = initial_camera(frame_input.viewport);
                    }
                    ui.add(Checkbox::new(&mut render_axes, "Show axes"));
                    ui.label("F is the blue axis\nR is the red axis\nU is the green axis");
                    ui.separator();

                    ui.heading("Debug");
                    if ui.button("Print cube to terminal").clicked() {
                        println!("{}", cube);
                    }
                });
                panel_width = gui_ctx.used_rect().width();
            },
        );

        let viewport = calc_viewport(panel_width, &frame_input);
        redraw |= camera.set_viewport(viewport);
        redraw |= mouse_control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            debug!("Drawing cube");
            let screen = frame_input.screen();
            let draw_res = screen
                .clear(ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0))
                .render(
                    &camera,
                    instanced_square.into_iter().chain(&inner_cube),
                    &[],
                )
                .write(|| {
                    if render_axes {
                        axes.render(&camera, &[]);
                    }

                    gui.render()
                });
            if let Err(e) = draw_res {
                error!("Error drawing cube {}", e);
            }
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
    Ok(())
}

fn initial_window() -> Result<Window, three_d::WindowError> {
    Window::new(WindowSettings {
        title: "Rusty Puzzle Cube!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
}

fn initial_camera(viewport: Viewport) -> Camera {
    Camera::new_perspective(
        viewport,
        vec3(3.0, 3.0, 6.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        50.0,
    )
}

fn initial_instances(ctx: &Context, cube: &Cube) -> Gm<InstancedMesh, ColorMaterial> {
    let instanced_square_mesh = InstancedMesh::new(ctx, &cube.to_instances(), &CpuMesh::square());
    Gm::new(
        instanced_square_mesh,
        ColorMaterial {
            color: Srgba::WHITE,
            ..Default::default()
        },
    )
}

fn inner_cube(ctx: &Context) -> Gm<Mesh, ColorMaterial> {
    Gm::new(
        Mesh::new(ctx, &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    )
}

fn calc_viewport(panel_width: f32, frame_input: &three_d::FrameInput) -> Viewport {
    if frame_input.viewport.width == 0 {
        frame_input.viewport
    } else {
        Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        }
    }
}