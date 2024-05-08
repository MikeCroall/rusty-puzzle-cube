use rusty_puzzle_cube::cube::Cube;
use three_d::egui::{special_emojis::GITHUB, Ui};

#[macro_export]
macro_rules! rotate_buttons {
    ($ui:ident, $cube:ident, $instanced_square:ident) => {
        rotate_buttons!($ui, $cube, $instanced_square, "F", Front);
        rotate_buttons!($ui, $cube, $instanced_square, "R", Right);
        rotate_buttons!($ui, $cube, $instanced_square, "U", Up);
        rotate_buttons!($ui, $cube, $instanced_square, "L", Left);
        rotate_buttons!($ui, $cube, $instanced_square, "B", Back);
        rotate_buttons!($ui, $cube, $instanced_square, "D", Down);
    };
    ($ui:ident, $cube:ident, $instanced_square:ident, $text:literal, $face:ident) => {
        $ui.horizontal(|ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Button,
                FontId::new(24.0, epaint::FontFamily::Proportional),
            );
            if ui.button($text).clicked() {
                $cube.rotate_face_90_degrees_clockwise(Face::$face);
                $instanced_square.set_instances(&$cube.to_instances());
            }
            if ui.button(format!("{}'", $text)).clicked() {
                $cube.rotate_face_90_degrees_anticlockwise(Face::$face);
                $instanced_square.set_instances(&$cube.to_instances());
            }
        });
    };
}

pub(super) fn header(ui: &mut Ui) {
    ui.heading("Rusty Puzzle Cube");
    ui.label("By Mike Croall");
    ui.hyperlink_to(
        format!("{GITHUB} on GitHub"),
        "https://github.com/MikeCroall/rusty-puzzle-cube/",
    );
    ui.separator();
}

pub(super) fn debug(ui: &mut Ui, cube: &Cube) {
    ui.heading("Debug");
    if ui.button("Print cube to terminal").clicked() {
        println!("{cube}");
    }
}
