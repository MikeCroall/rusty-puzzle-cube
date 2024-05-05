use three_d::{
    degrees, vec3, Axes, Camera, ClearState, ColorMaterial, CpuMesh, FrameOutput, Gm, Mesh,
    OrbitControl, Window, WindowSettings,
};

macro_rules! cubie_side {
    ($ctx:expr, $colour:ident, $($transform_fn:ident),*) => {
        {
            let mut cubie_side = Gm::new(
                Mesh::new($ctx, &CpuMesh::square()),
                ColorMaterial {
                    color: crate::colours::$colour,
                    ..Default::default()
                },
            );
            $(
                crate::transforms::$transform_fn(&mut cubie_side);
            )*
            cubie_side
        }
    };
}

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

    let blue_square = cubie_side!(&ctx, BLUE, translate_toward);
    let orange_square = cubie_side!(&ctx, ORANGE, quarter_turn_around_y, translate_right);
    let green_square = cubie_side!(&ctx, GREEN, translate_away);
    let red_square = cubie_side!(&ctx, RED, quarter_turn_around_y, translate_left);
    let white_square = cubie_side!(&ctx, WHITE, quarter_turn_around_x, translate_up);
    let yellow_square = cubie_side!(&ctx, YELLOW, quarter_turn_around_x, translate_down);

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
