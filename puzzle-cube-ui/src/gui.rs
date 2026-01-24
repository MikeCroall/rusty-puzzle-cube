mod anim_cube;
mod colours;
mod cube_3d_ext;
mod decided_move;
mod defaults;
mod gui_state;
mod mouse_control;
mod side_panel;
mod transforms;

#[cfg(not(target_arch = "wasm32"))]
mod file_io;

use crate::gui::{
    cube_3d_ext::PuzzleCube3D,
    defaults::{clear_state, initial_camera, initial_window},
    gui_state::GuiState,
    mouse_control::MouseControl,
};
use anim_cube::AnimCube;
use mouse_control::MouseControlOutput;
use rusty_puzzle_cube::{cube::Cube, known_transforms::KnownTransform};
use three_d::{
    Axes, ColorMaterial, Context, CpuMesh, Cull, FrameOutput, GUI, Gm, InstancedMesh, Mesh, Object,
    RenderStates, Srgba, Viewport,
};
use tracing::{debug, error};

const UNDO_QUEUE_MAX_SIZE: usize = 100;

pub(super) fn start_gui() -> anyhow::Result<()> {
    let window = initial_window()?;
    let mut state = GuiState::<AnimCube<Cube>, UNDO_QUEUE_MAX_SIZE>::init(&window)?;
    let mut mouse_control = MouseControl::new(state.camera.target(), 1.0, 80.0);
    let mut gui = GUI::new(&state.ctx);

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.;
        let mut redraw = frame_input.first_frame;
        redraw |= gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_ctx| {
                state.show_ui(gui_ctx, frame_input.viewport);
                panel_width = gui_ctx.used_rect().width();
            },
        );

        redraw |= state.camera.set_viewport(calc_viewport(
            panel_width,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
        ));

        let was_animating_before = state.cube.is_animating();
        let MouseControlOutput {
            redraw: needs_redraw_from_mouse,
            updated_cube,
            rotation_if_released_now,
        } = mouse_control.handle_events(&mut state, &mut frame_input.events);

        redraw |= state.rotation_if_released_now != rotation_if_released_now;
        state.rotation_if_released_now = rotation_if_released_now;

        if updated_cube || state.cube.is_animating() {
            state
                .cube
                .progress_animation(state.animation_speed * frame_input.elapsed_time);
            state.tiles.set_instances(&state.cube.as_instances());
        }
        let was_animating_after = state.cube.is_animating();
        redraw |= needs_redraw_from_mouse || was_animating_before || was_animating_after;

        if redraw {
            debug!("Drawing cube");
            let screen = frame_input.screen();
            if let Err(e) = screen
                .clear(clear_state())
                .render(&state.camera, &state.tiles, &[])
                .write(|| {
                    if state.render_axes {
                        Axes::new(&state.ctx, 0.05, 2.).render(&state.camera, &[]);
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

    KnownTransform::NestedCube4x4x4.perform_instantly(&mut cube);
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
