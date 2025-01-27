// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

use rand::{rngs::StdRng, SeedableRng};

fn gen_seed_from_time() -> [u8; 32] {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    macro_rules! wrap {
        ($a: expr) => {
            (($a as u128) % (u8::max_value() as u128)) as u8
        };
    }
    let mut s = [0u8; 32];
    s[0] = wrap!(now.as_millis());
    s[15] = wrap!(now.as_secs());
    s[31] = wrap!(now.as_nanos());
    s
}

pub fn initialize_generator(seed: u128, use_time: bool) -> StdRng {
    let mut s = [0u8; 32];
    if use_time {
        s = gen_seed_from_time();
    } else {
        s[..16].copy_from_slice(&seed.to_le_bytes());
        s[16..32].copy_from_slice(&seed.to_le_bytes());
    }
    SeedableRng::from_seed(s)
}
