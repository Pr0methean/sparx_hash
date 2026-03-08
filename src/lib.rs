use std::hash::Hasher;

/// The SPARX family of hash functions are named for the operations they use:
/// * Shift (`<<`)
/// * Popcount ([u128::count_ones])
/// * Addition ([u128::wrapping_add])
/// * Rotation ([u128::rotate_left])
/// * Xor (`^`)

pub mod sparx128;
pub mod sparx256;
pub mod sparx64;
#[cfg(test)]
mod generic_tests;

pub trait Hasher128: Hasher {
    fn finish128(&self) -> u128;
}