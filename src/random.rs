use once_cell::sync::Lazy;
use rand::{distributions::Standard, Rng};

const SEED: [u8; 32] = [0; 32];
static mut RNG: Lazy<rand::prelude::StdRng> = Lazy::new(|| rand::SeedableRng::from_seed(SEED));

pub fn rand_uniform() -> f32 {
    unsafe { RNG.sample(Standard) }
}
