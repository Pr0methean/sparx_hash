use std::hash::{BuildHasher, Hasher, RandomState};
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};
use digest::consts::U16;
use crate::sparx128::{permute_sparx128, Sparx128Hasher, SPARX128_INIT};

#[derive(Clone, Copy)]
pub struct Sparx256Hasher(u128, u128);

impl Default for Sparx256Hasher {
    fn default() -> Self {
        Self(SPARX128_INIT, SPARX128_INIT)
    }
}

impl Hasher for Sparx256Hasher {
    fn finish(&self) -> u64 {
        let mut out_hasher = Sparx128Hasher(self.0);
        out_hasher.write_u128(self.1);
        out_hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut bytes = bytes.iter().copied();
        while let Some(byte) = bytes.next() {
            let byte = u128::from(byte);
            let p0 = permute_sparx128(self.0.wrapping_add(byte));
            let other_byte = bytes.next().map(u128::from).unwrap_or(256);
            let p1 = permute_sparx128(self.1.rotate_left(59).wrapping_add(other_byte));
            self.0 = self.0.wrapping_add(p1);
            self.1 = self.1 ^ p0;
        }
    }
}

impl Update for Sparx256Hasher {
    fn update(&mut self, data: &[u8]) {
        self.write(data);
    }
}

impl OutputSizeUser for Sparx256Hasher { type OutputSize = U16; }

impl FixedOutput for Sparx256Hasher {
    fn finalize_into(self, out: &mut Output<Self>) {
        let mut transform = Sparx128Hasher(self.0);
        transform.write_u128(self.1);
        transform.finalize_into(out);
    }
}

impl HashMarker for Sparx256Hasher {}

impl From<&RandomState> for Sparx256Hasher {
    fn from(state: &RandomState) -> Self {
        Sparx256HashBuilder::from(state).build_hasher()
    }
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx256Hasher> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx256Hasher {
        use rand::RngExt;
        Sparx256Hasher(rng.random(), rng.random())
    }
}

#[derive(Clone, Copy)]
pub struct Sparx256HashBuilder(u128, u128);

impl BuildHasher for Sparx256HashBuilder {
    type Hasher = Sparx256Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        Sparx256Hasher(self.0, self.1)
    }
}

impl Default for Sparx256HashBuilder {
    fn default() -> Self {
        Self(SPARX128_INIT, SPARX128_INIT)
    }
}

impl From<&RandomState> for Sparx256HashBuilder {
    fn from(state: &RandomState) -> Self {
        Self(
            (state.hash_one("Sparx256") as u128) << 64 | state.hash_one("hasher") as u128,
            (state.hash_one("HASHER") as u128) << 64 | state.hash_one("256Sparx") as u128,
        )
    }
}


#[cfg(feature = "rand")]
impl rand::distr::Distribution<Sparx256HashBuilder> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sparx256HashBuilder {
        use rand::RngExt;
        Sparx256HashBuilder(rng.random(), rng.random())
    }
}

#[cfg(test)]
mod tests {
    use crate::generic_tests::{test_avalanche, test_unique_hashes};
    use crate::sparx256::Sparx256Hasher;

    #[test]
    fn test_unique_hashes_256() {
        test_unique_hashes(Sparx256Hasher::default);
    }

    #[test]
    fn test_avalanche_256() {
        test_avalanche(Sparx256Hasher::default);
    }
}