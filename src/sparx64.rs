use core::hash::{BuildHasher, Hasher};
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};
use digest::typenum::U8;

pub const SPARX64_INIT: u64 = 0x9E3779B97F4A7C15;
pub const SPARX64_FINALIZE: u64 = 0xf86c6a11d0c18e95;

pub fn permute_sparx64(input: u64) -> u64 {
        let count1 = input.count_ones();
        /* odd increment */
        let inc = ((1u64 << (count1 ^ 37)).wrapping_add(input).wrapping_add((count1 as u64).rotate_left(8))) | 1;
        let t = input
             ^ inc.rotate_left(13).wrapping_sub(input.rotate_left(29));
        t.wrapping_add(inc)
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx64Hasher> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx64Hasher {
        Sparx64Hasher(rng.next_u64())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sparx64Hasher(u64);

impl Hasher for Sparx64Hasher {
    fn finish(&self) -> u64 {
        permute_sparx64(self.0.wrapping_add(512).wrapping_mul(SPARX64_FINALIZE)) ^ self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut bytes = bytes.iter().copied();
        while let Some(byte) = bytes.next() {
            let input = (byte as u64).reverse_bits() | bytes.next().map(|b| b as u64).unwrap_or(256);
            self.0 = permute_sparx64(self.0.wrapping_add(input));
        }
    }
}

impl Default for Sparx64Hasher {
    fn default() -> Self {
        Self(SPARX64_INIT)
    }
}

#[cfg(feature = "std")]
impl From<&std::hash::RandomState> for Sparx64Hasher {
    fn from(state: &std::hash::RandomState) -> Self {
        Sparx64HashBuilder::from(state).build_hasher()
    }
}

pub struct Sparx64HashBuilder(u64);

impl BuildHasher for Sparx64HashBuilder {
    type Hasher = Sparx64Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Sparx64Hasher(self.0)
    }
}

impl Default for Sparx64HashBuilder {
    fn default() -> Self {
        Self(SPARX64_INIT)
    }
}

impl HashMarker for Sparx64Hasher {}

impl OutputSizeUser for Sparx64Hasher { type OutputSize = U8; }

impl Update for Sparx64Hasher {
    fn update(&mut self, data: &[u8]) {
        self.write(data);
    }
}

impl FixedOutput for Sparx64Hasher {
    fn finalize_into(self, out: &mut Output<Self>) {
        out.copy_from_slice(&self.finish().to_le_bytes());
    }
}

#[cfg(feature = "std")]
impl From<&std::hash::RandomState> for Sparx64HashBuilder {
    fn from(state: &std::hash::RandomState) -> Self {
        Self(state.hash_one("Sparx64"))
    }
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx64HashBuilder> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx64HashBuilder {
        Sparx64HashBuilder(rng.next_u64())
    }
}


#[cfg(test)]
mod tests {
    use crate::generic_tests::{test_avalanche, test_unique_hashes};
    use crate::sparx64::Sparx64Hasher;

    #[test]
    fn test_unique_hashes_64() {
        test_unique_hashes(Sparx64Hasher::default);
    }

    #[test]
    fn test_avalanche_64() {
        test_avalanche(Sparx64Hasher::default);
    }
}