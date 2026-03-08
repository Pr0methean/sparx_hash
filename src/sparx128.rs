use std::hash::{BuildHasher, Hasher, RandomState};
use crate::Hasher128;

pub const SPARX128_INIT: u128 = 0xf39cc0605cedc834_1082276bf3a27251;

pub fn permute_sparx128(input: u128) -> u128 {
        let count1 = input.count_ones() & 127;
        /* odd increment */
        let inc = ((1u128 << (count1 ^ 61)).wrapping_add((count1 as u128).rotate_left(8)).wrapping_add(input)) | 1;
        let t = input
            .wrapping_add(inc.rotate_left(29))
            .wrapping_sub(input.rotate_left(41));
        t.wrapping_add(inc ^ (1u128 << (count1 ^ 11)))
}

pub struct Sparx128Hasher(pub(crate) u128);

impl Default for Sparx128Hasher {
    fn default() -> Self {
        Self(SPARX128_INIT)
    }
}

impl Hasher for Sparx128Hasher {
    fn finish(&self) -> u64 {
        let out = self.finish128();
        out as u64 ^ (out >> 64) as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = permute_sparx128(self.0.wrapping_add(u128::from(*byte)));
        }
    }
}

impl Hasher128 for Sparx128Hasher {
    fn finish128(&self) -> u128 {
        self.0 ^ permute_sparx128(self.0.reverse_bits().wrapping_add(SPARX128_INIT))
    }
}

impl From<&RandomState> for Sparx128Hasher {
    fn from(state: &RandomState) -> Self {
        Sparx128HashBuilder::from(state).build_hasher()
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

impl From<&RandomState> for Sparx128HashBuilder {
    fn from(state: &RandomState) -> Self {
        Self((state.hash_one("Sparx128") as u128) << 64 | state.hash_one("128Sparx") as u128)
    }
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx128Hasher> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx128Hasher {
        use rand::RngExt;
        Sparx128Hasher(rng.random())
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