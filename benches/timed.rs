// Copyright (c) 2024-2026 Evelyn Lewis
// SPDX-License-Identifier: MIT

use divan::{Bencher, black_box};
use x800::one_shot;

mod shared {
    pub(super) const STARTUP_FILE: &str = "fuzz/corpus/roger/startup";
    pub(super) const GOLD_FILE: &str = "fuzz/corpus/roger/gold";
    pub(super) const SHORT_FILE: &str = "fuzz/corpus/roger/short";
}

#[divan::bench(sample_count = 10, sample_size = 1000)]
fn timed_startup(c: Bencher) {
    timed_file(c, shared::STARTUP_FILE);
}

#[divan::bench(sample_count = 10, sample_size = 1000)]
fn timed_short(c: Bencher) {
    timed_file(c, shared::SHORT_FILE);
}

#[divan::bench(sample_count = 10, sample_size = 1000)]
fn timed_gold(c: Bencher) {
    timed_file(c, shared::GOLD_FILE);
}

#[inline(always)]
#[allow(clippy::inline_always)]
fn timed_file(c: Bencher, file: &str) {
    let content = std::fs::read(file).unwrap();
    c.bench(|| one_shot(black_box(&content)));
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}
