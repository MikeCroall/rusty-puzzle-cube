mod colours;
mod cube_ext;
mod decided_move;
mod defaults;
#[cfg(not(target_arch = "wasm32"))]
mod file_io;
mod mouse_control;
mod side_panel;
mod transforms;

use crate::gui::{
    cube_ext::AsInstances,
    defaults::{clear_state, initial_camera, initial_window},
    mouse_control::MouseControl,
};
use mouse_control::MouseControlOutput;
use rusty_puzzle_cube::{cube::Cube, known_transforms::cube_in_cube_in_cube_in_cube};
use three_d::{
    Axes, ColorMaterial, Context, CpuMesh, Cull, FrameOutput, GUI, Gm, InstancedMesh, Mesh, Object,
    RenderStates, Srgba, Viewport, egui::ScrollArea,
};
use tracing::{debug, error, info};

pub(super) fn start_gui() -> anyhow::Result<()> {
    info!("Initialising default cube");
    let mut side_length = 4;
    let mut cube = Cube::create(side_length.try_into()?);
    cube_in_cube_in_cube_in_cube(&mut cube);

    info!("Initialising GUI");
    let window = initial_window()?;
    let mut camera = initial_camera(window.viewport());
    let mut mouse_control = MouseControl::new(camera.target(), 1.0, 80.0);
    let mut unreasonable_mode = false;

    let ctx = window.gl();
    let mut gui = GUI::new(&ctx);

    let mut tiles = initial_instances(&ctx, &cube);

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
                    ScrollArea::vertical().show(ui, |ui| {
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
                        #[cfg(not(target_arch = "wasm32"))]
                        side_panel::debug(
                            ui,
                            &cube,
                            &ctx,
                            frame_input.viewport,
                            &camera,
                            &tiles,
                            &inner_cube,
                        );
                    })
                });
                panel_width = gui_ctx.used_rect().width();
            },
        );

        let viewport = calc_viewport(
            panel_width,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
        );
        redraw |= camera.set_viewport(viewport);

        let MouseControlOutput {
            redraw: needs_redraw,
            updated_cube,
        } = mouse_control.handle_events(
            &ctx,
            &inner_cube,
            side_length,
            &mut camera,
            &mut frame_input.events,
            &mut cube,
        );
        if updated_cube {
            tiles.set_instances(&cube.as_instances());
        }
        redraw |= needs_redraw;

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
    let instanced_square_mesh = InstancedMesh::new(ctx, &cube.as_instances(), &CpuMesh::cube());
    let material = ColorMaterial {
        color: Srgba::WHITE,
        render_states: RenderStates {
            cull: Cull::Back,
            ..Default::default()
        },
        ..Default::default()
    };
    Gm::new(instanced_square_mesh, material)
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

#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn calc_viewport(panel_width: f32, viewport: Viewport, device_pixel_ratio: f32) -> Viewport {
    if viewport.width == 0 {
        viewport
    } else {
        Viewport {
            x: (panel_width * device_pixel_ratio) as i32,
            y: 0,
            width: viewport.width - (panel_width * device_pixel_ratio) as u32,
            height: viewport.height,
        }
    }
}

#[cfg(test)]
mod tests {
    use three_d::Viewport;

    use super::calc_viewport;

    #[test]
    fn test_valid_viewport_when_window_minimized() {
        let minimized_viewport = Viewport {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let _ = calc_viewport(50., minimized_viewport, 1.);
    }
}
