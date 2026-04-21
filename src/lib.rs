#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use digest::Digest;
use digest::typenum::U16;

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
#[cfg(feature = "rng")]
pub mod rng;

pub trait DigestU128 {
    fn finalize_u128(&self) -> u128;
}

impl <T: Digest<OutputSize = U16> + Clone> DigestU128 for T {
    fn finalize_u128(&self) -> u128 {
        let mut output = [0u8; 16];
        self.clone().finalize_into((&mut output).into());
        u128::from_le_bytes(output)
    }
}