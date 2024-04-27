use crate::colours::{BLUE, GREEN, RED};
use std::f32::consts::PI;

use three_d::{
    degrees, radians, vec3, Axes, Camera, ClearState, ColorMaterial, CpuMesh, FrameOutput, Gm,
    Mat4, Mesh, OrbitControl, Window, WindowSettings,
};

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

    let mut blue_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: BLUE,
            ..Default::default()
        },
    );
    blue_square.set_transformation(Mat4::from_translation(vec3(-0.5, 0., 1.5)));

    let mut green_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: GREEN,
            ..Default::default()
        },
    );
    green_square.set_transformation(
        Mat4::from_angle_x(radians(0.5 * PI)) * Mat4::from_translation(vec3(-0.5, 0.5, -1.)),
    );

    let mut red_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: RED,
            ..Default::default()
        },
    );
    red_square.set_transformation(
        Mat4::from_angle_y(radians(0.5 * PI)) * Mat4::from_translation(vec3(-0.5, 0., 0.5)),
    );

    let axes = Axes::new(&context, 0.1, 2.);

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        mouse_control.handle_events(&mut camera, &mut frame_input.events);
        // gpu_model.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0))
            .render(
                &camera,
                blue_square
                    .into_iter()
                    .chain(&green_square)
                    .chain(&red_square)
                    .chain(&axes),
                &[],
            );

        FrameOutput::default()
    });
}
