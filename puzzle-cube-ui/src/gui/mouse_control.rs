use rusty_puzzle_cube::cube::face::Face;
use three_d::{
    pick, Camera, ColorMaterial, Context, Event, Gm, Mesh, MouseButton, OrbitControl, Vec3, Vector3,
};
use tracing::info;

pub(super) struct MouseControl {
    orbit: OrbitControl,
}

impl MouseControl {
    pub(super) fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            orbit: OrbitControl::new(target, min_distance, max_distance),
        }
    }

    pub(super) fn handle_events(
        &mut self,
        ctx: &Context,
        inner_cube: &Gm<Mesh, ColorMaterial>,
        side_length: usize,
        camera: &mut Camera,
        events: &mut [Event],
    ) -> bool {
        // todo mouse control cube, or camera orbit if cube itself not interacted

        for event in events.iter_mut() {
            #[allow(clippy::single_match)]
            match event {
                Event::MouseMotion {
                    button,
                    position,
                    handled,
                    ..
                } => {
                    if *button == Some(MouseButton::Left) {
                        if let Some(pick) = pick(ctx, camera, *position, inner_cube) {
                            let face = pick_to_face(pick);
                            info!("Pick {face:?} with side_length {side_length}, supressing camera movement");
                            *handled = true;
                        }
                    }
                }
                _ => {}
            }
        }

        self.orbit.handle_events(camera, events)
    }
}

fn pick_to_face(pick: Vector3<f32>) -> Face {
    const EPSILON: f32 = 0.0001;
    if (pick.x - 1.).abs() < EPSILON {
        Face::Right
    } else if (pick.x + 1.).abs() < EPSILON {
        Face::Left
    } else if (pick.y - 1.).abs() < EPSILON {
        Face::Up
    } else if (pick.y + 1.).abs() < EPSILON {
        Face::Down
    } else if (pick.z - 1.).abs() < EPSILON {
        Face::Front
    } else if (pick.z + 1.).abs() < EPSILON {
        Face::Back
    } else {
        panic!("pick_to_face interaction found no valid face from pick. This should never happen with inner cube.");
    }
}
