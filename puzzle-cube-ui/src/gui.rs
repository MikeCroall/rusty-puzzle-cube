use crate::colours::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW};
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
    blue_square.set_transformation(Mat4::from_translation(vec3(0., 0., 1.)));

    let mut orange_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: ORANGE,
            ..Default::default()
        },
    );
    orange_square.set_transformation(
        Mat4::from_translation(vec3(1., 0., 0.)) * Mat4::from_angle_y(radians(0.5 * PI)),
    );

    let mut green_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: GREEN,
            ..Default::default()
        },
    );
    green_square.set_transformation(Mat4::from_translation(vec3(0., 0., -1.)));

    let mut red_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: RED,
            ..Default::default()
        },
    );
    red_square.set_transformation(
        Mat4::from_translation(vec3(-1., 0., 0.)) * Mat4::from_angle_y(radians(0.5 * PI)),
    );

    let mut white_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: WHITE,
            ..Default::default()
        },
    );
    white_square.set_transformation(
        Mat4::from_translation(vec3(0., 1., 0.)) * Mat4::from_angle_x(radians(0.5 * PI)),
    );

    let mut yellow_square = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: YELLOW,
            ..Default::default()
        },
    );
    yellow_square.set_transformation(
        Mat4::from_translation(vec3(0., -1., 0.)) * Mat4::from_angle_x(radians(0.5 * PI)),
    );

    let axes = Axes::new(&context, 0.05, 2.);

    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= mouse_control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            println!("Redrawing {}", frame_input.accumulated_time);
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0))
                .render(
                    &camera,
                    blue_square
                        .into_iter()
                        .chain(&orange_square)
                        .chain(&green_square)
                        .chain(&red_square)
                        .chain(&white_square)
                        .chain(&yellow_square)
                        .chain(&axes),
                    &[],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}
