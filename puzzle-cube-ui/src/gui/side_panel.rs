use std::fmt::Display;

use crate::gui::{
    GuiState, anim_cube::AnimationProgress, cube_3d_ext::PuzzleCube3D, initial_camera,
};
use rusty_puzzle_cube::{cube::side_lengths::SideLength, known_transforms::KnownTransform};
use strum::IntoEnumIterator;
use three_d::{
    Viewport,
    egui::{
        Button, Checkbox, ComboBox, Context, ProgressBar, Rgba, ScrollArea, SidePanel, Slider, Ui,
        special_emojis::GITHUB,
    },
};

const MIN_CUBE_SIZE: usize = 1;
const MAX_CUBE_SIZE: usize = 100;
const EXTRA_SPACING: f32 = 10.;

impl<C: PuzzleCube3D + Display, const UNDO_SIZE: usize> GuiState<C, UNDO_SIZE> {
    pub(crate) fn show_ui(&mut self, gui_ctx: &Context, viewport: Viewport) {
        SidePanel::left("side_panel").show(gui_ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                Self::header(ui);
                ui.separator();

                self.initialise_cube(ui);
                ui.separator();

                self.control_cube(ui);
                ui.separator();

                self.control_camera(ui, viewport);
                ui.separator();

                #[cfg(not(target_arch = "wasm32"))]
                self.debug_ctrls(ui, viewport);
            })
        });
    }

    fn header(ui: &mut Ui) {
        ui.heading("Rusty Puzzle Cube");
        ui.label("By Mike Croall");
        ui.hyperlink_to(
            format!("{GITHUB} on GitHub"),
            "https://github.com/MikeCroall/rusty-puzzle-cube/",
        );
        ui.add_space(EXTRA_SPACING);
    }

    fn initialise_cube(&mut self, ui: &mut Ui) {
        ui.add_space(EXTRA_SPACING);
        ui.heading("Initialise Cube");
        ui.add_space(EXTRA_SPACING);

        let prev_side_length = self.side_length;
        ui.add(Slider::new(
            &mut self.side_length,
            MIN_CUBE_SIZE..=MAX_CUBE_SIZE,
        ));
        ui.add_space(EXTRA_SPACING);

        if ui
            .button(format!(
                "New {prev_side_length}x{prev_side_length}x{prev_side_length} Cube"
            ))
            .clicked()
        {
            let side_length = SideLength::try_from(self.side_length)
                .expect("UI is configured to only allow selecting valid side length values");
            self.cube = self.cube.recreate_at_size(side_length);
            self.undo_queue.clear();
            self.tiles.set_instances(&self.cube.as_instances());
        }
        ui.add_space(EXTRA_SPACING);
    }

    fn control_cube(&mut self, ui: &mut Ui) {
        ui.add_space(EXTRA_SPACING);
        ui.heading("Cube Controls");
        ui.label("Click and drag directly on the cube to make a rotation");
        ui.label("You must only drag across one face of the cube");
        ui.label(
            "Dragging to another face, diagonally, or for a very small distance will be cancelled",
        );
        ui.add_space(EXTRA_SPACING);

        ui.horizontal(|ui| {
            let undo_text = if self.undo_queue.is_full() {
                format!("Undo ({}, at limit)", self.undo_queue.len())
            } else if !self.undo_queue.is_empty() {
                format!("Undo ({})", self.undo_queue.len())
            } else {
                "Undo".to_owned()
            };
            if ui
                .add_enabled(!self.undo_queue.is_empty(), Button::new(undo_text))
                .clicked()
            {
                let to_undo = self
                    .undo_queue
                    .pop_back()
                    .expect("button disabled if queue empty");
                self.cube
                    .rotate(!to_undo)
                    .expect("moves on queue must be reversible");
            }

            if ui
                .add_enabled(!self.undo_queue.is_empty(), Button::new("Undo all"))
                .clicked()
            {
                let moves = self.undo_queue.to_vec();
                self.undo_queue.clear();
                self.cube
                    .rotate_seq_with_progress(moves.into_iter().rev().map(|r| !r))
                    .expect("moves on queue must be reversible");
            }
        });
        ui.add_space(EXTRA_SPACING);

        let shuffle_moves = self.cube.side_length() * 10;
        if ui
            .button(format!("Shuffle ({shuffle_moves} moves)"))
            .clicked()
        {
            self.cube.shuffle(shuffle_moves);
            self.cube.cancel_animation();
            self.undo_queue.clear();
            self.tiles.set_instances(&self.cube.as_instances());
        }
        ui.add_space(EXTRA_SPACING);

        ui.label("Pre-defined transforms");
        ComboBox::from_label("")
            .selected_text(self.selected_transform.name())
            .show_ui(ui, |ui| {
                for known_transform in KnownTransform::iter() {
                    ui.selectable_value(
                        &mut self.selected_transform,
                        known_transform,
                        known_transform.name(),
                    );
                }
            });
        ui.label(self.selected_transform.description());
        ui.add_space(EXTRA_SPACING);

        if ui
            .add_enabled(
                self.selected_transform
                    .minimum_side_length()
                    .is_none_or(|min_len| self.cube.side_length() >= min_len),
                Button::new("Perform transform"),
            )
            .clicked()
        {
            self.cube
                .rotate_seq_with_progress(self.selected_transform.sequence().into_iter())
                .expect("Known transforms must use valid sequences");
        }
        ui.add_space(EXTRA_SPACING);

        if let Some(progress) = self
            .cube
            .animation_progress()
            .and_then(AnimationProgress::sequence_linear_with_sub_step)
        {
            ui.add(ProgressBar::new(progress).show_percentage());
            ui.add_space(EXTRA_SPACING);
        }
    }

    fn control_camera(&mut self, ui: &mut Ui, viewport: Viewport) {
        ui.add_space(EXTRA_SPACING);
        ui.heading("Camera and Rendering");
        ui.label("The camera can be moved with a click and drag starting from the blank space around the cube, or by dragging from one face to any other face or empty space");
        ui.add_space(EXTRA_SPACING);

        if ui.button("Reset camera").clicked() {
            self.camera = initial_camera(viewport);
        }
        ui.add_space(EXTRA_SPACING);

        if ui
            .add(Checkbox::new(&mut self.lock_upright, "Lock upright"))
            .changed()
            && self.lock_upright
        {
            self.camera = initial_camera(viewport);
        }
        ui.add_space(EXTRA_SPACING);

        ui.add(Checkbox::new(&mut self.render_axes, "Show axes"));
        if self.render_axes {
            ui.colored_label(Rgba::from_rgb(0.15, 0.15, 1.), "F is the blue axis");
            ui.colored_label(Rgba::RED, "R is the red axis");
            ui.colored_label(Rgba::GREEN, "U is the green axis");
        }
        ui.add_space(EXTRA_SPACING);

        ui.label("Animation speed");
        ui.add(Slider::new(&mut self.animation_speed, 0.1..=3.0));
        ui.add_space(EXTRA_SPACING);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod non_wasm {
    use std::fmt::Display;

    use crate::gui::{GuiState, cube_3d_ext::PuzzleCube3D, file_io, side_panel::EXTRA_SPACING};
    use three_d::{Viewport, egui::Ui};
    use tracing::{error, info};

    impl<C: PuzzleCube3D + Display, const UNDO_SIZE: usize> GuiState<C, UNDO_SIZE> {
        pub(crate) fn debug_ctrls(&mut self, ui: &mut Ui, viewport: Viewport) {
            ui.add_space(EXTRA_SPACING);
            ui.heading("Debug");
            ui.add_space(EXTRA_SPACING);

            if ui.button("Print cube to terminal").clicked() {
                info!("\n{}", self.cube);
            }
            ui.add_space(EXTRA_SPACING);

            if ui.button("Save as image").clicked() {
                if let Err(e) =
                    file_io::save_as_image(&self.ctx, viewport, &self.camera, &self.tiles)
                {
                    error!("Could not save image file: {}", e);
                }
            }
            ui.add_space(EXTRA_SPACING);
        }
    }
}
