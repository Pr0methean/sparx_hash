use core::convert::Infallible;
use core::num::Wrapping;
use core::hash::Hasher;
use digest::Digest;
use rand::rand_core::block::{BlockRng, Generator};
use rand::{SeedableRng, TryRng};
use crate::DigestU128;
use crate::sparx128::Sparx128Hasher;
use crate::sparx256::Sparx256Hasher;
use crate::sparx64::Sparx64Hasher;

pub struct CounterHashRngCore<T: Digest> {
    hasher: T,
    counter: Wrapping<u128>,
}
impl <T: Digest + DigestU128> Generator for CounterHashRngCore<T> {
    type Output = [u64; 2];

    fn generate(&mut self, output: &mut Self::Output) {
        self.hasher.update(self.counter.0.to_le_bytes());
        self.counter += 1;
        let out_u128 = self.hasher.finalize_u128();
        output[0] = out_u128 as u64;
        output[1] = (out_u128 >> 64) as u64;
    }
}

impl Generator for CounterHashRngCore<Sparx64Hasher> {
    type Output = [u64; 1];

    fn generate(&mut self, output: &mut Self::Output) {
        self.hasher.update(self.counter.0.to_le_bytes());
        self.counter += 1;
        output[0] = self.hasher.finish();
    }
}

pub struct CounterHashRng<T: Digest>(BlockRng<CounterHashRngCore<T>>) where CounterHashRngCore<T>: Generator;

impl <T: Digest, const N: usize> TryRng for CounterHashRng<T> where CounterHashRngCore<T>: Generator<Output = [u64; N]> {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let next_u64 = self.0.next_word();
        Ok(next_u64 as u32 ^ (next_u64 >> 32) as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.0.next_word())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        Ok(self.0.fill_bytes(dst))
    }
}

const GOLDEN_RATIO_MULTIPLIER: Wrapping<u128> = Wrapping(0x9e37_79b9_7f4a_7c15_f39c_c060_5ced_c835);

impl SeedableRng for CounterHashRng<Sparx64Hasher> {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut hasher = Sparx64Hasher::default();
        hasher.write(&seed);
        let counter = Wrapping(hasher.clone().finish() as u128) * GOLDEN_RATIO_MULTIPLIER;
        Self(BlockRng::new(CounterHashRngCore { hasher, counter }))
    }
}

impl SeedableRng for CounterHashRng<Sparx128Hasher> {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut hasher = Sparx128Hasher::default();
        hasher.write(&seed);
        let counter = Wrapping(hasher.clone().finish() as u128) * GOLDEN_RATIO_MULTIPLIER;
        Self(BlockRng::new(CounterHashRngCore { hasher, counter }))
    }
}

impl SeedableRng for CounterHashRng<Sparx256Hasher> {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut hasher = Sparx256Hasher::default();
        hasher.write(&seed);
        let counter = Wrapping(hasher.clone().finish() as u128) * GOLDEN_RATIO_MULTIPLIER;
        Self(BlockRng::new(CounterHashRngCore { hasher, counter }))
    }
}