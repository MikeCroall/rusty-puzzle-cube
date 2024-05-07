# Rusty Puzzle Cube

## An experimental implementation of simple puzzle cubes in Rust

[![rust-ci](https://github.com/MikeCroall/rusty-puzzle-cube/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/MikeCroall/rusty-puzzle-cube/actions)
[![lib coverage](https://img.shields.io/codecov/c/github/MikeCroall/rusty-puzzle-cube?flag=lib&style=flat&label=lib%20coverage)](https://codecov.io/gh/MikeCroall/rusty-puzzle-cube?flags[0]=lib)
[![ui coverage](https://img.shields.io/codecov/c/github/MikeCroall/rusty-puzzle-cube?flag=ui&style=flat&label=gui%20coverage)](https://codecov.io/gh/MikeCroall/rusty-puzzle-cube?flags[0]=ui)

### Puzzle Cube GUI Crate

Experimental, still a work in progress, etc.

#### Cube in Cube in Cube (3x3 algo only on multiple cube sizes)

![Cube in Cube in Cube 3d 3x3 screenshot](img/3x3-3d-cicic.png)
![Cube in Cube in Cube 3d 4x4 screenshot](img/4x4-3d-cicic.png)
![Cube in Cube in Cube 3d 10x10 screenshot](img/10x10-3d-cicic.png)

#### Controls

Unreasonable mode simply changes the maximum cube size from 100 to 2000

![Controls for the 3d renderer](img/controls-3d.png)

### Puzzle Cube Lib Crate Demo

Demos of basic 3x3 notation being parsed and applied to a newly created cube

#### Cube in Cube in Cube

```rust
let mut cube = Cube::create(3);
let sequence = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
perform_3x3_sequence(sequence, &mut cube).unwrap();
print!("{cube}");
```

![Cube in Cube in Cube output screenshot](img/cube-in-cube-in-cube.png)

#### Checkerboard Corners

```rust
let mut cube = Cube::create(3);
let sequence = "R2 L2 F2 B2 U2 D2";
perform_3x3_sequence(sequence, &mut cube).unwrap();
print!("{cube}");
```

![Checkerboard Corners output screenshot](img/checkerboard-corners.png)

#### Unique Cubies and Large Cubes

Large cubes can be created by providing a larger side length, and cubies can each be given a unique character to keep track of exactly where they move as moves are applied

Note that side length is limited to a maximum of 8 when using unique characters to avoid leaving the basic ASCII range (and trying to use the DEL control code in a cubie)

```rust
let mut cube = Cube::create_with_unique_characters(8);
print!("{cube}");
```

![Big Cube and Unique Cubie output screenshot](img/big-cube-unique-cubie.png)

Note that large cubes do not currently support any moves that a 3x3 does not support.
For example, rotating only the center column of a 5x5, or the 2nd column of a 4x4 is currently impossible
