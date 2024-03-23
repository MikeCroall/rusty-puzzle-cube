use enum_map::{enum_map, EnumMap};
use itertools::izip;

use self::cubie_colour::CubieColour;
use self::face::{Face, IndexAlignment};

mod cubie_colour;
pub(crate) mod face;

type Side = Vec<Vec<CubieColour>>;

const HORIZONTAL_PADDING: &str = " ";

pub(crate) struct Cube {
    side_map: EnumMap<Face, Box<Side>>,
}

impl Cube {
    pub(crate) fn create(side_length: usize) -> Self {
        Self {
            side_map: enum_map! {
                Face::Top => Box::new(create_side_with_unique_characters(side_length, &CubieColour::White)),
                Face::Bottom => Box::new(create_side_with_unique_characters(side_length, &CubieColour::Yellow)),
                Face::Front => Box::new(create_side_with_unique_characters(side_length, &CubieColour::Blue)),
                Face::Right => Box::new(create_side_with_unique_characters(side_length, &CubieColour::Orange)),
                Face::Back => Box::new(create_side_with_unique_characters(side_length, &CubieColour::Green)),
                Face::Left => Box::new(create_side_with_unique_characters(side_length, &CubieColour::Red)),
            },
        }
    }

    pub(crate) fn rotate_face_90_degrees_clockwise(&mut self, face: Face) {
        println!("Simulating rotating {:?} face 90 degrees clockwise", face);
        self.rotate_face_90_degrees_clockwise_without_adjacents(face);
        self.rotate_face_90_degrees_clockwise_only_adjacents(face);
    }

    fn rotate_face_90_degrees_clockwise_without_adjacents(&mut self, face: Face) {
        let side: &mut Vec<Vec<CubieColour>> = &mut self.side_map[face];
        side.reverse();
        for i in 1..side.len() {
            let (left, right) = side.split_at_mut(i);
            (0..i).for_each(|j| {
                std::mem::swap(&mut left[j][i], &mut right[0][j]);
            });
        }
    }

    fn rotate_face_90_degrees_clockwise_only_adjacents(&mut self, face: Face) {
        let adjacents = face.adjacent_faces_clockwise();
        let slice_0 = get_clockwise_slice_of_side(&self.side_map[adjacents[0].0], &adjacents[0].1);
        let slice_1 = get_clockwise_slice_of_side(&self.side_map[adjacents[1].0], &adjacents[1].1);
        let slice_2 = get_clockwise_slice_of_side(&self.side_map[adjacents[2].0], &adjacents[2].1);
        let slice_3 = get_clockwise_slice_of_side(&self.side_map[adjacents[3].0], &adjacents[3].1);

        let final_order = {
            let mut preliminary_order = adjacents.iter();
            let first_element = preliminary_order.next();
            preliminary_order
                .chain(first_element)
                .collect::<Vec<&(Face, IndexAlignment)>>()
        };

        println!("\tOrder to write to: {:?}", final_order); // todo remove prints when copy_adjacent_over finished
        println!("\t\t{:?} should take {:?}", final_order[0], slice_0);
        println!("\t\t{:?} should take {:?}", final_order[1], slice_1);
        println!("\t\t{:?} should take {:?}", final_order[2], slice_2);
        println!("\t\t{:?} should take {:?}", final_order[3], slice_3);

        // todo sometimes needs reversed values - see output
        self.copy_adjacent_over(final_order[0], slice_0);
        self.copy_adjacent_over(final_order[1], slice_1);
        self.copy_adjacent_over(final_order[2], slice_2);
        self.copy_adjacent_over(final_order[3], slice_3);
    }

    fn copy_adjacent_over(
        &mut self,
        (target_face, target_alignment): &(Face, IndexAlignment),
        values: Vec<CubieColour>,
    ) {
        let side = &mut self.side_map[*target_face];
        match target_alignment {
            IndexAlignment::OuterStart => println!(
                // todo (can change to println! for testing output of other branches etc.)
                "This index alignment is not implemented yet for copy_adjacent_over - still WIP"
            ),
            IndexAlignment::OuterEnd => println!(
                // todo (can change to println! for testing output of other branches etc.)
                "This index alignment is not implemented yet for copy_adjacent_over - still WIP"
            ),
            IndexAlignment::InnerFirst => {
                side.first_mut()
                    .expect("Side had no inner")
                    .clone_from_slice(&values);
            }
            IndexAlignment::InnerLast => {
                side.last_mut()
                    .expect("Side had no inner")
                    .clone_from_slice(&values);
            }
        }
    }

    pub(crate) fn print_cube(&self) {
        self.print_indented_single_side(Face::Top);
        self.print_unindented_four_sides(Face::Left, Face::Front, Face::Right, Face::Back);
        self.print_indented_single_side(Face::Bottom);
    }

    fn print_indented_single_side(&self, face: Face) {
        let side = &*self.side_map[face];
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
            print!(
                "{}{}",
                cubie.get_coloured_display_char(),
                HORIZONTAL_PADDING
            );
        }
    }
}

fn create_side_with_unique_characters(
    side_length: usize,
    colour_variant_creator: &dyn Fn(Option<char>) -> CubieColour,
) -> Side {
    if side_length > 8 {
        panic!("create_side_with_unique_characters does not support side_length > 8")
    }
    let mut side = vec![];
    for outer in 0..side_length {
        let mut inner_vec = vec![];
        for inner in 0..side_length {
            let value = ((side_length * outer) + inner) as u32;
            let display_char = char::from_u32('0' as u32 + value);
            inner_vec.push(colour_variant_creator(display_char));
        }
        side.push(inner_vec);
    }
    side
}

fn get_clockwise_slice_of_side(side: &Side, index_alignment: &IndexAlignment) -> Vec<CubieColour> {
    match index_alignment {
        IndexAlignment::OuterStart => side
            .iter()
            .map(|inner| inner.first().expect("Side inner had no member").to_owned())
            .collect::<Vec<CubieColour>>(),
        IndexAlignment::OuterEnd => side
            .iter()
            .map(|inner| inner.last().expect("Side inner had no member").to_owned())
            .rev()
            .collect::<Vec<CubieColour>>(),
        IndexAlignment::InnerFirst => {
            let mut inner_first_vec = side.first().expect("Side had no inner").to_owned();
            inner_first_vec.reverse();
            inner_first_vec
        }
        IndexAlignment::InnerLast => side.last().expect("Side had no inner").to_owned(),
    }
}
