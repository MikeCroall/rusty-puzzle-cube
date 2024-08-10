# Rusty Puzzle Cube

## An experimental implementation of simple puzzle cubes in Rust

[![rust-ci](https://github.com/MikeCroall/rusty-puzzle-cube/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/MikeCroall/rusty-puzzle-cube/actions)
[![lib coverage](https://img.shields.io/codecov/c/github/MikeCroall/rusty-puzzle-cube?flag=lib&style=flat&label=lib%20coverage)](https://codecov.io/gh/MikeCroall/rusty-puzzle-cube?flags[0]=lib)
[![ui coverage](https://img.shields.io/codecov/c/github/MikeCroall/rusty-puzzle-cube?flag=ui&style=flat&label=gui%20coverage)](https://codecov.io/gh/MikeCroall/rusty-puzzle-cube?flags[0]=ui)

### Puzzle Cube GUI Crate

Experimental, still a work in progress, etc.

#### Cube in Cube in Cube (3x3x3 algo only on multiple cube sizes)

![Cube in Cube in Cube 3D 3x3x3 screenshot](img/3x3x3-3D-cicic.png)
![Cube in Cube in Cube 3D 4x4x4 screenshot](img/4x4x4-3D-cicic.png)
![Cube in Cube in Cube 3D 10x10x10 screenshot](img/10x10x10-3D-cicic.png)
![Shuffled 3D 10x10x10 screenshot](img/10x10x10-3D-shuffle.png)

#### Controls

Click and drag along the edge of a face to perform a rotation

Unreasonable mode simply changes the maximum cube size from 100 to 2000

Shuffle will make `10n` moves on an `n` x `n` x `n` cube

Some controls are removed on the WASM target

![Controls for the 3D renderer](img/gui-controls-3D.png)

### Building Puzzle Cube GUI Crate for web

Build command written from inside the `web` dir

```bash
npx wasm-pack build "../puzzle-cube-ui" --target web --out-name web --out-dir ../web/pkg
```

Files built into `web/pkg/`, which can be hosted by

```bash
npm run serve
```

Then visit `http://localhost:8080`

### Puzzle Cube Lib Crate Demo

Demos of basic notation being parsed and applied to a newly created cube

#### Cube in Cube in Cube

```rust
let mut cube = Cube::create(3);
let sequence = "F R' U' F' U L' B U' B2 U' F' R' B R2 F U L U";
perform_sequence(sequence, &mut cube).unwrap();
print!("{cube}");
```

![Cube in Cube in Cube output screenshot](img/cube-in-cube-in-cube.png)

#### Checkerboard Corners

```rust
let mut cube = Cube::create(3);
let sequence = "R2 L2 F2 B2 U2 D2";
perform_sequence(sequence, &mut cube).unwrap();
print!("{cube}");
```

![Checkerboard Corners output screenshot](img/checkerboard-corners.png)

#### Unique Cubies and Large Cubes

Large cubes can be created by providing a larger side length, and cubies can each be given a unique character to keep track of exactly where they move as moves are applied

Note that side length is limited to a maximum of 8 when using unique characters to avoid leaving the basic ASCII range (and trying to use the DEL control code in a cubie)

```rust
let mut cube = Cube::create_with_unique_characters(8.try_into()?);
print!("{cube}");
```

![Big Cube and Unique Cubie output screenshot](img/big-cube-unique-cubie.png)
