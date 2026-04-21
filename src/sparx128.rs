use core::hash::{BuildHasher, Hasher};
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};
use digest::typenum::U16;

pub const SPARX128_INIT: u128 = 0xf39cc0605cedc834_1082276bf3a27251;

pub fn permute_sparx128(input: u128) -> u128 {
        let count1 = input.count_ones();
        /* odd increment */
        let inc = ((1u128 << (count1 ^ 61)).wrapping_add((count1 as u128).rotate_left(8)).wrapping_add(input)) | 1;
        let t = input
            ^ inc.rotate_left(29)
            .wrapping_sub(input.rotate_right(41));
        t.wrapping_add(inc)
}

#[derive(Copy, Clone, Debug)]
pub struct Sparx128Hasher(pub(crate) u128);

impl Default for Sparx128Hasher {
    fn default() -> Self {
        Self(SPARX128_INIT)
    }
}

impl Hasher for Sparx128Hasher {
    fn finish(&self) -> u64 {
        let out = self.0 ^ permute_sparx128(self.0.reverse_bits().wrapping_add(SPARX128_INIT));
        out as u64 ^ (out >> 64) as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut bytes = bytes.iter().copied();
        while let Some(byte) = bytes.next() {
            let input = (byte as u128).reverse_bits() | bytes.next().map(|b| b as u128).unwrap_or(256);
            self.0 = permute_sparx128(self.0.wrapping_add(input));
        }
    }
}

impl Update for Sparx128Hasher {
    fn update(&mut self, data: &[u8]) {
        self.write(data);
    }
}

impl HashMarker for Sparx128Hasher {}

impl OutputSizeUser for Sparx128Hasher { type OutputSize = U16; }

impl FixedOutput for Sparx128Hasher {
    fn finalize_into(self, out: &mut Output<Self>) {
        let out_u128 = self.0 ^ permute_sparx128(self.0.reverse_bits().wrapping_add(SPARX128_INIT));
        out.copy_from_slice(&out_u128.to_le_bytes());
    }
}

#[cfg(feature = "std")]
impl From<&std::hash::RandomState> for Sparx128Hasher {
    fn from(state: &std::hash::RandomState) -> Self {
        Sparx128HashBuilder::from(state).build_hasher()
    }
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx128Hasher> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx128Hasher {
        use rand::RngExt;
        Sparx128Hasher(rng.random())
    }
}

pub struct Sparx128HashBuilder(u128);

impl BuildHasher for Sparx128HashBuilder {
    type Hasher = Sparx128Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Sparx128Hasher(self.0)
    }
}

impl Default for Sparx128HashBuilder {
    fn default() -> Self {
        Self(SPARX128_INIT)
    }
}

#[cfg(feature = "std")]
impl From<&std::hash::RandomState> for Sparx128HashBuilder {
    fn from(state: &std::hash::RandomState) -> Self {
        Self((state.hash_one("Sparx128") as u128) << 64 | state.hash_one("128Sparx") as u128)
    }
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx128HashBuilder> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx128HashBuilder {
        use rand::RngExt;
        Sparx128HashBuilder(rng.random())
    }
}

#[cfg(test)]
mod tests {
    use crate::generic_tests::{test_avalanche, test_unique_hashes};
    use crate::sparx128::Sparx128Hasher;

    #[test]
    fn test_unique_hashes_128() {
        test_unique_hashes(Sparx128Hasher::default);
    }

    #[test]
    fn test_avalanche_128() {
        test_avalanche(Sparx128Hasher::default);
    }
}