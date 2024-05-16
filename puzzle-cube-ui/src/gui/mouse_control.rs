use three_d::{Camera, Event, OrbitControl, Vec3};

pub(super) struct MouseControl {
    orbit: OrbitControl,
}

impl MouseControl {
    pub(super) fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            orbit: OrbitControl::new(target, min_distance, max_distance),
        }
    }

    pub(super) fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        // todo mouse control cube, or camera orbit if cube itself not interacted

        self.orbit.handle_events(camera, events)
    }
}
