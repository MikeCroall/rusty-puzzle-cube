use self::cubie_colour::CubieColour;

mod cubie_colour;

type Side = Vec<Vec<CubieColour>>;

const CUBIE_CHAR: char = '■';
const HORIZONTAL_PADDING: &str = " ";

pub(crate) struct Cube {
    top: Side,
    bottom: Side,
    front: Side,
    back: Side,
    left: Side,
    right: Side,
}

impl Cube {
    pub(crate) fn create(side_length: usize) -> Self {
        Self {
            top: vec![vec![CubieColour::White; side_length]; side_length],
            bottom: vec![vec![CubieColour::Yellow; side_length]; side_length],
            front: vec![vec![CubieColour::Blue; side_length]; side_length],
            right: vec![vec![CubieColour::Orange; side_length]; side_length],
            back: vec![vec![CubieColour::Green; side_length]; side_length],
            left: vec![vec![CubieColour::Red; side_length]; side_length],
        }
    }

    pub(crate) fn print_cube(&self) {
        self.print_top_row();
        self.print_middle_row();
        self.print_bottom_row();
    }

    fn print_top_row(&self) {
        self.print_indented_single_side(&self.top);
    }

    fn print_middle_row(&self) {
        self.print_unindented_four_sides(&self.left, &self.front, &self.right, &self.back);
    }

    fn print_bottom_row(&self) {
        self.print_indented_single_side(&self.bottom);
    }

    fn print_indented_single_side(&self, side: &Side) {
        let side_length = side.len();
        for cubie_row in side {
            print!("{}", format!(" {}", HORIZONTAL_PADDING).repeat(side_length));
            self.print_cubie_row(cubie_row);
            println!();
        }
    }

    fn print_unindented_four_sides(
        &self,
        side_a: &Side,
        side_b: &Side,
        side_c: &Side,
        side_d: &Side,
    ) {
        let prezip_left = side_a.iter().zip(side_b.iter());
        let prezip_right = side_c.iter().zip(side_d.iter());
        let zipped = prezip_left.zip(prezip_right);
        for ((cubie_row_a, cubie_row_b), (cubie_row_c, cubie_row_d)) in zipped {
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
