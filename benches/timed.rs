// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

use divan::{black_box, Bencher};
use std::fs;
use x800::one_shot;

mod shared;
#[divan::bench(sample_count = 10, sample_size = 1000)]
pub fn timed_startup(c: Bencher) {
    timed_file(c, shared::STARTUP_FILE);
}

#[divan::bench(sample_count = 10, sample_size = 1000)]
pub fn timed_short(c: Bencher) {
    timed_file(c, shared::SHORT_FILE);
}

#[divan::bench(sample_count = 10, sample_size = 1000)]
pub fn timed_gold(c: Bencher) {
    timed_file(c, shared::GOLD_FILE);
}

#[inline(always)]
fn timed_file(c: Bencher, file: &str) {
    let content = fs::read(file).unwrap();
    c.bench(|| one_shot(black_box(&content)));
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}
