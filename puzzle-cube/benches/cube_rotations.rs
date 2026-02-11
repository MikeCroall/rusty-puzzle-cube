use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use itertools::Itertools;
use rusty_puzzle_cube::cube::{
    Cube, PuzzleCube, direction::Direction, face::Face, rotation::Rotation,
};
use strum::{EnumIter, IntoEnumIterator};

const MOVES_TO_MAKE_PER_SAMPLE: usize = 30;

#[derive(EnumIter)]
enum BasicRotationKind {
    Faces,
    Slices,
}

fn bench_id(direction: &Direction, basic_rotation_kind: &BasicRotationKind) -> String {
    let direction = match direction {
        Direction::Clockwise => "clockwise",
        Direction::Anticlockwise => "anticlockwise",
    };
    let basic_rotation_kind = match basic_rotation_kind {
        BasicRotationKind::Faces => "face",
        BasicRotationKind::Slices => "slice",
    };
    format!("{MOVES_TO_MAKE_PER_SAMPLE} {direction} {basic_rotation_kind} turns")
}

fn gen_seq(
    basic_rotation_kind: &BasicRotationKind,
    direction: &Direction,
    side_length: usize,
) -> Vec<Rotation> {
    match basic_rotation_kind {
        BasicRotationKind::Faces => Face::iter()
            .map(move |f| match direction {
                Direction::Clockwise => Rotation::clockwise(f),
                Direction::Anticlockwise => Rotation::anticlockwise(f),
            })
            .cycle()
            .take(MOVES_TO_MAKE_PER_SAMPLE)
            .collect(),
        BasicRotationKind::Slices => Face::iter()
            .flat_map(move |face| {
                (1..(side_length - 1)).map(move |layer| match direction {
                    Direction::Clockwise => Rotation::clockwise_setback_from(face, layer),
                    Direction::Anticlockwise => Rotation::anticlockwise_setback_from(face, layer),
                })
            })
            .cycle()
            .take(MOVES_TO_MAKE_PER_SAMPLE)
            .collect(),
    }
}

fn benchmark_nxnxn_cube(c: &mut Criterion, n: usize) {
    let mut group = c.benchmark_group(format!("{n}x{n}x{n} cube"));

    for (direction, basic_rotation_kind) in [Direction::Clockwise, Direction::Anticlockwise]
        .iter()
        .cartesian_product(BasicRotationKind::iter())
    {
        group.bench_function(bench_id(direction, &basic_rotation_kind), move |b| {
            b.iter_batched(
                || {
                    (
                        Cube::create(n.try_into().unwrap()),
                        gen_seq(&basic_rotation_kind, direction, n),
                    )
                },
                |(mut cube, seq)| {
                    cube.rotate_seq(black_box(seq)).unwrap();
                },
                BatchSize::SmallInput,
            );
        });
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
