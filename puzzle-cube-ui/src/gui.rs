use three_d::*;

pub(super) fn start_gui() {
    let window = Window::new(WindowSettings {
        title: "Rusty Puzzle Cube!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .expect("Must be able to create window");

    let context = window.gl();

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

    let cube_colour = Srgba::RED;
    let cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: cube_colour,
                ..Default::default()
            },
        ),
    );

    let bounding_box_cube = Gm::new(
        BoundingBox::new_with_thickness(&context, cube.aabb(), 0.01),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );

    let directional_light =
        DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(-4.0, -4.0, 4.0));
    let ambient_light = AmbientLight::new(&context, 0.5, Srgba::WHITE);

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        mouse_control.handle_events(&mut camera, &mut frame_input.events);
        // gpu_model.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                cube.into_iter().chain(&bounding_box_cube),
                &[&ambient_light, &directional_light],
            );

        FrameOutput::default()
    });
}
