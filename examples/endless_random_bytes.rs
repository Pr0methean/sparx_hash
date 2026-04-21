use rand::{Rng, SeedableRng};
use sparx_hash::rng::CounterHashRng;
use sparx_hash::sparx256::Sparx256Hasher;
use std::io::{Write, stdout};
fn main() {
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).unwrap();
    eprintln!("Seed: {}", seed.map(|b| format!("{:02X}", b)).join(""));
    let mut prng = CounterHashRng::<Sparx256Hasher>::from_seed(seed);
    let mut stdout = stdout().lock();
    loop {
        let mut buffer = [0u8; 1 << 16];
        prng.fill_bytes(&mut buffer);
        if let Err(e) = stdout.write_all(&buffer) {
            eprintln!("Error writing to stdout: {}", e);
            return;
        }
    }
}