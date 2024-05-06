use crate::cube_ext::instances::ToInstances;
use rusty_puzzle_cube::{cube::Cube, known_transforms::cube_in_cube_in_cube};
use three_d::{
    degrees, vec3, Axes, Camera, ClearState, ColorMaterial, CpuMesh, FrameOutput, Gm,
    InstancedMesh, OrbitControl, Srgba, Window, WindowSettings,
};

pub(super) fn start_gui() {
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
    let mut cube = Cube::create(4);
    cube_in_cube_in_cube(&mut cube);

    let instances = cube.to_instances();
    let instanced_square_mesh = InstancedMesh::new(&ctx, &instances, &CpuMesh::square());
    let instanced_square = Gm::new(
        instanced_square_mesh,
        ColorMaterial {
            color: Srgba::WHITE,
            ..Default::default()
        },
    );

    let axes = Axes::new(&ctx, 0.05, 2.);

    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= mouse_control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            println!("Redrawing {}", frame_input.accumulated_time);
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0))
                .render(&camera, instanced_square.into_iter().chain(&axes), &[]);
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
