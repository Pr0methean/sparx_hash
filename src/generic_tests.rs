use alloc::collections::BTreeSet;
use core::hash::Hasher;

pub fn test_unique_hashes<T: Hasher>(hasher_creator: impl Fn() -> T) {
    let mut hashes = BTreeSet::new();
    hashes.insert(hasher_creator().finish());
    for i in 0..=u8::MAX {
        let mut hasher = hasher_creator();
        hasher.write_u8(i);
        let hash = hasher.finish();
        assert!(hashes.insert(hash), "Hash {hash} of {i} already exists");
    }
    for i in 0..=u16::MAX {
        let mut hasher = hasher_creator();
        hasher.write_u16(i);
        let hash = hasher.finish();
        assert!(hashes.insert(hash), "Hash {hash} of {i} already exists");
    }
    for i in 0..=(1 << 24) {
        let mut hasher = hasher_creator();
        hasher.write_u32(i);
        let hash = hasher.finish();
        assert!(hashes.insert(hash), "Hash {hash} of {i} already exists");
    }
}

pub fn test_avalanche<T: Hasher>(hasher_creator: impl Fn() -> T) {
    const MIN_HAMMING_DISTANCE: u32 = 16;
    let mut total_distance = 0;
    let hasher = hasher_creator();
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
    assert!(average_distance >= 31.0);
    assert!(average_distance <= 33.0);
}