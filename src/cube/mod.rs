use enum_map::{enum_map, EnumMap};
use itertools::izip;

use self::cubie_colour::CubieColour;
use self::face::Face;

mod cubie_colour;
mod face;

type Side = Vec<Vec<CubieColour>>;

const CUBIE_CHAR: char = '■';
const HORIZONTAL_PADDING: &str = " ";

pub(crate) struct Cube {
    side_map: EnumMap<Face, Side>,
}

impl Cube {
    pub(crate) fn create(side_length: usize) -> Self {
        Self {
            side_map: enum_map! {
                Face::Top => vec![vec![CubieColour::White; side_length]; side_length],
                Face::Bottom => vec![vec![CubieColour::Yellow; side_length]; side_length],
                Face::Front => vec![vec![CubieColour::Blue; side_length]; side_length],
                Face::Right => vec![vec![CubieColour::Orange; side_length]; side_length],
                Face::Back => vec![vec![CubieColour::Green; side_length]; side_length],
                Face::Left => vec![vec![CubieColour::Red; side_length]; side_length],
            },
        }
    }

    pub(crate) fn print_cube(&self) {
        self.print_indented_single_side(Face::Top);
        self.print_unindented_four_sides(Face::Left, Face::Front, Face::Right, Face::Back);
        self.print_indented_single_side(Face::Bottom);
    }

    fn print_indented_single_side(&self, face: Face) {
        let side = &self.side_map[face];
        let side_length = side.len();
        for cubie_row in side {
            print!("{}", format!(" {}", HORIZONTAL_PADDING).repeat(side_length));
            self.print_cubie_row(cubie_row);
            println!();
        }
    }

    fn print_unindented_four_sides(&self, face_a: Face, face_b: Face, face_c: Face, face_d: Face) {
        let side_a = self.side_map[face_a].iter();
        let side_b = self.side_map[face_b].iter();
        let side_c = self.side_map[face_c].iter();
        let side_d = self.side_map[face_d].iter();

        for (cubie_row_a, cubie_row_b, cubie_row_c, cubie_row_d) in
            izip!(side_a, side_b, side_c, side_d)
        {
            self.print_cubie_row(cubie_row_a);
            self.print_cubie_row(cubie_row_b);
            self.print_cubie_row(cubie_row_c);
            self.print_cubie_row(cubie_row_d);
            println!();
        }
    }

    fn print_cubie_row(&self, cubie_row: &Vec<CubieColour>) {
        for cubie in cubie_row {
            print!("{}{}", cubie.colourise_char(CUBIE_CHAR), HORIZONTAL_PADDING);
        }
    }
}
