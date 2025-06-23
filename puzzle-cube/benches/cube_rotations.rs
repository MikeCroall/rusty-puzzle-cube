use std::{fmt::Display, hint::black_box};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rusty_puzzle_cube::cube::{
    Cube, PuzzleCube, direction::Direction, face::Face, rotation::Rotation,
};
use strum::{EnumIter, IntoEnumIterator};

struct DisplayDirection(Direction);

impl Display for DisplayDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            Direction::Clockwise => "Clockwise",
            Direction::Anticlockwise => "Anticlockwise",
        })
    }
}

#[derive(EnumIter)]
enum RotGen {
    Faces,
    Slices,
}

impl RotGen {
    const FACE_COUNT: usize = 6;
    const TO_TAKE_MULTIPLIER: usize = 100;

    fn to_take(side_length: usize) -> usize {
        let slice_count = Self::FACE_COUNT * (side_length - 2);
        slice_count * Self::TO_TAKE_MULTIPLIER
    }

    fn format_name(&self, side_length: usize) -> String {
        let to_take = Self::to_take(side_length);
        format!(
            "{} ({to_take} turns)",
            match self {
                RotGen::Faces => "faces",
                RotGen::Slices => "slices",
            }
        )
    }

    fn generate(
        &self,
        direction: Direction,
        side_length: usize,
    ) -> Box<dyn Iterator<Item = Rotation>> {
        let faces = Face::iter().map(move |f| match direction {
            Direction::Clockwise => Rotation::clockwise(f),
            Direction::Anticlockwise => Rotation::anticlockwise(f),
        });

        let slices = Face::iter().flat_map(move |face| {
            (1..(side_length - 1)).map(move |layer| match direction {
                Direction::Clockwise => Rotation::clockwise_setback_from(face, layer),
                Direction::Anticlockwise => Rotation::anticlockwise_setback_from(face, layer),
            })
        });

        let to_take = Self::to_take(side_length);

        match self {
            RotGen::Faces => Box::new(faces.cycle().take(to_take)),
            RotGen::Slices => Box::new(slices.cycle().take(to_take)),
        }
    }

    fn run(&self, direction: Direction, side_length: usize) {
        let rot = self.generate(direction, side_length);

        Cube::create(side_length.try_into().unwrap())
            .rotate_seq(rot)
            .unwrap();
    }

    fn bench_with_input(
        &self,
        group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
        side_length: usize,
        direction: Direction,
    ) {
        group.bench_with_input(
            BenchmarkId::new(self.format_name(side_length), DisplayDirection(direction)),
            &direction,
            |b, direction| b.iter(|| self.run(*direction, black_box(side_length))),
        );
    }
}

fn benchmark_nxnxn_cube(c: &mut Criterion, n: usize) {
    let mut group = c.benchmark_group(format!("{n}x{n}x{n} cube"));

    for direction in [Direction::Clockwise, Direction::Anticlockwise] {
        for rot_gen in RotGen::iter() {
            rot_gen.bench_with_input(&mut group, n, direction);
        }
    }

    group.finish();
}

fn benchmark_3x3x3_cube(c: &mut Criterion) {
    benchmark_nxnxn_cube(c, 3);
}

fn benchmark_15x15x15_cube(c: &mut Criterion) {
    benchmark_nxnxn_cube(c, 15);
}

fn benchmark_50x50x50_cube(c: &mut Criterion) {
    benchmark_nxnxn_cube(c, 50);
}

criterion_group!(benches_3x3x3, benchmark_3x3x3_cube);
criterion_group!(benches_15x15x15, benchmark_15x15x15_cube);
criterion_group!(benches_50x50x50, benchmark_50x50x50_cube);

criterion_main!(benches_3x3x3, benches_15x15x15, benches_50x50x50);
