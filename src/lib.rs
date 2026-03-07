use std::hash::Hasher;

const SPARX64_INIT: u64 = 0x9E3779B97F4A7C15;
const SPARX128_INIT: u128 = 0xf39cc0605cedc8341082276bf3a27251;
const SPARX64_FINALIZE: u64 = 0xf86c6a11d0c18e95;

/// The SPARX family of hash functions are named for the operations they use:
/// * Shift (`<<`)
/// * Popcount ([u128::count_ones])
/// * Addition ([u128::wrapping_add])
/// * Rotation ([u128::rotate_left])
/// * Xor (`^`)
fn permute_sparx128(input: u128) -> u128 {
        const SHIFT_MOD: u32 = 61;
        let count = input.count_ones();
        let shift = (count % SHIFT_MOD) + 8;

        /* odd increment */
        let inc = ((1u128 << (16 + SHIFT_MOD - shift)).wrapping_add((input.wrapping_add((count as u128).rotate_left(7))) << 1) )| 1;
        let t = input
            .wrapping_add(inc.rotate_left(13))
            .wrapping_add(input.rotate_left(37));
        t.wrapping_add(inc ^ (1u128 << shift))
}

fn permute_sparx64(input: u64) -> u64 {
        const SHIFT_MOD: u32 = 37;
        let count = input.count_ones();
        let shift = (count % SHIFT_MOD) + 7;

        /* odd increment */
        let inc = ((1u64 << (14 + SHIFT_MOD - shift)).wrapping_add((input.wrapping_add((count as u64).rotate_left(7))) << 1) )| 1;
        let t = input
            .wrapping_add(inc.rotate_left(13))
            .wrapping_add(input.rotate_left(29));
        t.wrapping_add(inc ^ (1u64 << shift))
}

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
        for byte in bytes {
            let new_state = self.0 ^ permute_sparx128(self.1.wrapping_add(256 + u128::from(*byte)));
            self.1 ^= u128::from(*byte).wrapping_add(permute_sparx128(self.0));
            self.0 = new_state;
        }
    }
}

pub struct Sparx128Hasher(u128);

impl Default for Sparx128Hasher {
    fn default() -> Self {
        Self(SPARX128_INIT)
    }
}

impl Hasher for Sparx128Hasher {
    fn finish(&self) -> u64 {
        let out = self.0 ^ permute_sparx128(self.0.wrapping_add(1024).reverse_bits());
        out as u64 ^ (out >> 64) as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = permute_sparx128(self.0.wrapping_add(256 + u128::from(*byte)));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_unique_hashes<T: Hasher>(hasher_creator: impl Fn() -> T) {
        let mut hashes = std::collections::HashSet::new();
        for i in 0..=u8::MAX {
            let mut hasher = hasher_creator();
            hasher.write_u8(i);
            assert!(hashes.insert(hasher.finish()));
        }
        for i in 0..=u16::MAX {
            let mut hasher = hasher_creator();
            hasher.write_u16(i);
            assert!(hashes.insert(hasher.finish()));
        }
        for i in 0..=(1 << 24) {
            let mut hasher = hasher_creator();
            hasher.write_u32(i);
            assert!(hashes.insert(hasher.finish()));
        }
    }

    fn test_avalanche<T: Hasher>(hasher_creator: impl Fn() -> T) {
        const MIN_HAMMING_DISTANCE: u32 = 16;
        let mut total_distance = 0;
        let mut hasher = hasher_creator();
        let empty_hash = hasher.finish();
        let mut hasher = hasher_creator();
        hasher.write_u128(0);
        let zero_hash = hasher.finish();
        let distance = (empty_hash ^ zero_hash).count_ones();
        assert!(distance >= MIN_HAMMING_DISTANCE);
        total_distance += distance;
        for i in 0..=127 {
            let input = 1u128 << i;
            let mut hasher = hasher_creator();
            hasher.write_u128(input);
            let hash = hasher.finish();
            let distance = (hash ^ zero_hash).count_ones();
            assert!(distance >= MIN_HAMMING_DISTANCE);
            total_distance += distance;
            let distance = (hash ^ empty_hash).count_ones();
            assert!(distance >= MIN_HAMMING_DISTANCE);
            total_distance += distance;
        }
        let average_distance = total_distance as f64 / 257.0;
        println!("Average distance: {}", average_distance);
        assert!(average_distance >= 31.0);
        assert!(average_distance <= 33.0);
    }

    #[test]
    fn test_unique_hashes_64() {
        test_unique_hashes(Sparx64Hasher::default);
    }

    #[test]
    fn test_unique_hashes_128() {
        test_unique_hashes(Sparx128Hasher::default);
    }

    #[test]
    fn test_unique_hashes_256() {
        test_unique_hashes(Sparx256Hasher::default);
    }

    #[test]
    fn test_avalanche_64() {
        test_avalanche(Sparx64Hasher::default);
    }

    #[test]
    fn test_avalanche_128() {
        test_avalanche(Sparx128Hasher::default);
    }

    #[test]
    fn test_avalanche_256() {
        test_avalanche(Sparx256Hasher::default);
    }
}
