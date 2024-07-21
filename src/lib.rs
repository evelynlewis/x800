// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

// Include README.md docs inline
// These are rendered to the root Crate page
#![doc = include_str!("../README.md")]

use std::mem;

mod board;
mod colour;
mod play;
use play::play;

pub fn driver(input: &[u8], seed: u64) {
    fastrand::seed(seed);
    play(&play::Input::Slice(input));
}

/// # Panics
///
/// Will panic if `data.len()` is less than `8`
#[inline(always)]
pub fn one_shot(data: &[u8]) {
    type Seed = u64;
    if data.len() < mem::size_of::<Seed>() + 1 {
        return;
    }
    // Generate 64 bit PRNG seed
    let seed = Seed::from_le_bytes(
        data[..mem::size_of::<Seed>()]
            .try_into()
            .expect("Failed to generate seed"),
    );
    driver(&data[(mem::size_of::<Seed>() - 1)..], seed);
}
