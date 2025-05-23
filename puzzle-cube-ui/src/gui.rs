mod anim_cube;
mod colours;
mod cube_3d_ext;
mod decided_move;
mod defaults;
#[cfg(not(target_arch = "wasm32"))]
mod file_io;
mod mouse_control;
mod side_panel;
mod transforms;

use crate::gui::{
    cube_3d_ext::PuzzleCube3D,
    defaults::{clear_state, initial_camera, initial_window},
    mouse_control::MouseControl,
};
use anim_cube::AnimCube;
use circular_buffer::CircularBuffer;
use mouse_control::MouseControlOutput;
use rusty_puzzle_cube::{
    cube::{Cube, rotation::Rotation},
    known_transforms::cube_in_cube_in_cube_in_cube,
};
use side_panel::draw_side_panel;
use three_d::{
    Axes, ColorMaterial, Context, CpuMesh, Cull, FrameOutput, GUI, Gm, InstancedMesh, Mesh, Object,
    RenderStates, Srgba, Viewport,
};
use tracing::{debug, error, info};

const UNDO_QUEUE_MAX_SIZE: usize = 100;

pub(super) fn start_gui() -> anyhow::Result<()> {
    info!("Initialising default cube");
    let mut side_length = 4;
    let mut cube = initial_anim_cube(side_length)?;

    info!("Initialising GUI");
    let window = initial_window()?;
    let mut camera = initial_camera(window.viewport());
    let mut lock_upright = false;
    let mut mouse_control = MouseControl::new(camera.target(), 1.0, 80.0);

    let ctx = window.gl();
    let mut gui = GUI::new(&ctx);

    let mut tiles = initial_instances(&ctx, &cube);

    let pick_cube = inner_cube(&ctx);

    let mut render_axes = false;
    let axes = Axes::new(&ctx, 0.05, 2.);
    let mut animation_speed = 1.0;

    let mut undo_queue = CircularBuffer::<UNDO_QUEUE_MAX_SIZE, Rotation>::new();

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.;
        let mut redraw = frame_input.first_frame;
        redraw |= gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_ctx| {
                draw_side_panel(
                    &mut side_length,
                    &mut cube,
                    &mut undo_queue,
                    &mut camera,
                    &mut lock_upright,
                    &ctx,
                    &mut tiles,
                    &pick_cube,
                    &mut render_axes,
                    &mut animation_speed,
                    frame_input.viewport,
                    gui_ctx,
                );
                panel_width = gui_ctx.used_rect().width();
            },
        );

        redraw |= camera.set_viewport(calc_viewport(
            panel_width,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
        ));

        let MouseControlOutput {
            redraw: needs_redraw_from_mouse,
            updated_cube,
        } = mouse_control.handle_events(
            &ctx,
            &pick_cube,
            side_length,
            &mut camera,
            lock_upright,
            &mut frame_input.events,
            &mut cube,
            &mut undo_queue,
        );
        if updated_cube || cube.is_animating() {
            cube.progress_animation(animation_speed * frame_input.elapsed_time);
            tiles.set_instances(&cube.as_instances());
        }
        redraw |= needs_redraw_from_mouse | cube.is_animating();

        if redraw {
            debug!("Drawing cube");
            let screen = frame_input.screen();
            if let Err(e) = screen
                .clear(clear_state())
                .render(&camera, &tiles, &[])
                .write(|| {
                    if render_axes {
                        axes.render(&camera, &[]);
                    }

                    gui.render()
                })
            {
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

fn initial_anim_cube(side_length: usize) -> anyhow::Result<AnimCube<Cube>> {
    let mut cube = AnimCube::new(Cube::create(side_length.try_into()?));

    cube_in_cube_in_cube_in_cube(&mut cube);
    cube.cancel_animation();

    Ok(cube)
}

fn initial_instances<I: PuzzleCube3D>(ctx: &Context, cube: &I) -> Gm<InstancedMesh, ColorMaterial> {
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
