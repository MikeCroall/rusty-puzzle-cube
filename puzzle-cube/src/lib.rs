#![warn(missing_docs, missing_debug_implementations)]
//! Crate providing a puzzle cube implementation, with the ability to apply string-encoded sequences of moves.

/// Module providing the core cube implementation.
pub mod cube;

/// Module providing some pre-defined patterns that can be applied to a cube.
pub mod known_transforms;

/// Module providing the ability to parse string-encoded sequences of moves and apply them to a cube.
pub mod notation;

/// Property testing.
#[cfg(test)]
mod quickcheck;
