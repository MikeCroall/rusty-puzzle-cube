use three_d::{Camera, ClearState, Viewport, Window, WindowSettings, degrees, vec3};

pub(super) fn initial_window() -> Result<Window, three_d::WindowError> {
    Window::new(WindowSettings {
        title: "Rusty Puzzle Cube!".to_string(),
        ..Default::default()
    })
}

pub(super) fn initial_camera(viewport: Viewport) -> Camera {
    Camera::new_perspective(
        viewport,
        vec3(2.0, 2.5, 4.0),
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use three_d::Vector3;

    #[test]
    fn test_initial_camera_targets_origin() {
        let camera = initial_camera(Viewport {
            x: 0,
            y: 0,
            width: 75,
            height: 50,
        });

        assert_eq!(camera.target(), Vector3::new(0., 0., 0.));
    }

    #[test]
    fn test_clear_state_is_monochrome() {
        let clear_state = clear_state();

        assert_eq!(clear_state.red, clear_state.green);
        assert_eq!(clear_state.red, clear_state.blue);
        assert_eq!(clear_state.alpha, Some(1.));
        assert_eq!(clear_state.depth, Some(1.));
    }
}
