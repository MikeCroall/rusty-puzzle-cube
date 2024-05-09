use three_d::{degrees, vec3, Camera, ClearState, Viewport, Window, WindowSettings};

pub(super) fn initial_window() -> Result<Window, three_d::WindowError> {
    Window::new(WindowSettings {
        title: "Rusty Puzzle Cube!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
}

pub(super) fn initial_camera(viewport: Viewport) -> Camera {
    Camera::new_perspective(
        viewport,
        vec3(3.0, 3.0, 6.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    )
}

pub(super) fn clear_state() -> ClearState {
    ClearState::color_and_depth(0.13, 0.13, 0.13, 1.0, 1.0)
}
