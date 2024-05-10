mod colours;
mod cube_ext;
mod defaults;
#[cfg(not(target_arch = "wasm32"))]
mod file_io;
mod side_panel;
mod transforms;

use crate::gui::{
    cube_ext::ToInstances,
    defaults::{clear_state, initial_camera, initial_window},
};
use rusty_puzzle_cube::{cube::Cube, known_transforms::cube_in_cube_in_cube};
use three_d::{
    Axes, ColorMaterial, Context, CpuMesh, FrameOutput, Gm, InstancedMesh, Mesh, Object,
    OrbitControl, Srgba, Viewport, GUI,
};
use tracing::{debug, error, info};

use self::transforms::scale_down_inner_cube;

pub(super) fn start_gui() -> Result<(), three_d::WindowError> {
    info!("Initialising default cube");
    let mut side_length = 3;
    let mut cube = Cube::create(side_length);
    cube_in_cube_in_cube(&mut cube);

    info!("Initialising GUI");
    let window = initial_window()?;
    let mut camera = initial_camera(window.viewport());
    let mut mouse_control = OrbitControl::new(*camera.target(), 1.0, 80.0);
    let mut unreasonable_mode = false;

    let ctx = window.gl();
    let mut gui = GUI::new(&ctx);
    let mut tiles = initial_instances(&ctx, &cube);

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
                use three_d::egui::SidePanel;
                SidePanel::left("side_panel").show(gui_ctx, |ui| {
                    side_panel::header(ui);
                    side_panel::initialise_cube(
                        ui,
                        &mut unreasonable_mode,
                        &mut side_length,
                        &mut cube,
                        &mut tiles,
                    );
                    side_panel::control_cube(ui, &mut cube, &mut tiles);
                    side_panel::control_camera(
                        ui,
                        &mut camera,
                        frame_input.viewport,
                        &mut render_axes,
                    );
                    side_panel::debug(
                        ui,
                        &cube,
                        &ctx,
                        frame_input.viewport,
                        &camera,
                        &tiles,
                        &inner_cube,
                    );
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
                .clear(clear_state())
                .render(&camera, tiles.into_iter().chain(&inner_cube), &[])
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
    let mut inner_cube = Gm::new(
        Mesh::new(ctx, &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );
    inner_cube.set_transformation(scale_down_inner_cube());
    inner_cube
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
