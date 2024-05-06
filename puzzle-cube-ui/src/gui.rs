use crate::cube_ext::instances::ToInstances;
use rusty_puzzle_cube::{cube::Cube, known_transforms::cube_in_cube_in_cube};
use three_d::{
    degrees, vec3, Axes, Camera, ClearState, ColorMaterial, CpuMesh, FrameOutput, Gm,
    InstancedMesh, Mesh, OrbitControl, Srgba, Window, WindowSettings,
};
use tracing::info;

pub(super) fn start_gui() {
    tracing_subscriber::fmt::init();

    let window = Window::new(WindowSettings {
        title: "Rusty Puzzle Cube!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .expect("Must be able to create window");
    let ctx = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0, 3.0, 6.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        50.0,
    );

    let mut mouse_control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // todo use InstancedMesh.set_instances each time cube changes?
    let mut cube = Cube::create(5);
    cube_in_cube_in_cube(&mut cube);

    let instanced_square_mesh = InstancedMesh::new(&ctx, &cube.to_instances(), &CpuMesh::square());
    let instanced_square = Gm::new(
        instanced_square_mesh,
        ColorMaterial {
            color: Srgba::WHITE,
            ..Default::default()
        },
    );

    // todo could make inner cube instances for each (external facing) cubie to make rotation animations less funky, when I actually add them...
    let inner_cube = Gm::new(
        Mesh::new(&ctx, &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );

    let axes = Axes::new(&ctx, 0.05, 2.);

    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= mouse_control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            info!("Redrawing cube");
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0))
                .render(
                    &camera,
                    instanced_square.into_iter().chain(&inner_cube).chain(&axes),
                    &[],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
