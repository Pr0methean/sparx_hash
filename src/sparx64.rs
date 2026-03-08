use std::hash::{BuildHasher, Hasher, RandomState};

pub const SPARX64_INIT: u64 = 0x9E3779B97F4A7C15;
pub const SPARX64_FINALIZE: u64 = 0xf86c6a11d0c18e95;

pub fn permute_sparx64(input: u64) -> u64 {
        let count1 = input.count_ones() & 127;
        /* odd increment */
        let inc = ((1u64 << (count1 ^ 37)).wrapping_add(input).wrapping_add((count1 as u64).rotate_left(8))) | 1;
        let t = input
            .wrapping_add(inc.rotate_left(13))
            .wrapping_sub(input.rotate_left(29));
        t.wrapping_add(inc ^ (1 << (count1 ^ 7)))
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx64Hasher> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx64Hasher {
        Sparx64Hasher(rng.next_u64())
    }
}

pub struct Sparx64Hasher(u64);

impl Hasher for Sparx64Hasher {
    fn finish(&self) -> u64 {
        permute_sparx64(self.0.wrapping_add(512).wrapping_mul(SPARX64_FINALIZE)) ^ self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = permute_sparx64(self.0.wrapping_add(256 + u64::from(*byte)));
        }
    }
}

impl Default for Sparx64Hasher {
    fn default() -> Self {
        Self(SPARX64_INIT)
    }
}

impl From<&RandomState> for Sparx64Hasher {
    fn from(state: &RandomState) -> Self {
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

impl From<&RandomState> for Sparx64HashBuilder {
    fn from(state: &RandomState) -> Self {
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