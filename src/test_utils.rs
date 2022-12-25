//! Contains code commonly repeated in my unit tests
//! I know even having the word "util" in your source code gets some software
//! engineers foaming at the mouth, and they may have a point. I don't see this
//! particular module as an issue, but if you feel differently please feel free
//! to let me know why - maybe I'll improve my code and learn something :)

#![cfg(test)]

use rand::{Rng, SeedableRng};

pub const RAND_SEED: u64 = 123;
pub const RAND_ITERATIONS: usize = 1000;

/// Makes a new random number generator
pub fn make_rng() -> impl Rng {
    rand_chacha::ChaCha8Rng::seed_from_u64(RAND_SEED)
}

/// Generates a pseudo-random character, designed to trigger as many weird
/// edge-cases as possible >:)
pub fn gen_rand_char(rng: &mut impl Rng) -> char {
    // Most of the time, we'll just use a normal ASCII value...
    if rng.gen_bool(0.75) {
        rng.gen_range(' '..='~')
    }
    // ...but every now and then, let's mix things up
    else {
        // Sometimes with a completely random character
        if rng.gen_bool(0.9) {
            rng.gen::<char>()
        }
        // And other times with a weird control character
        else {
            rng.gen_range('\0'..=' ')
        }
    }
}
