/*
  Copyright (c) 2024 Evelyn Lewis

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
*/

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roger::one_shot;
use std::fs;

mod shared;

pub fn criterion_startup(c: &mut Criterion) {
    criterion_file(c, shared::STARTUP_FILE, "startup file");
}

pub fn criterion_short(c: &mut Criterion) {
    criterion_file(c, shared::SHORT_FILE, "short file");
}

pub fn criterion_gold(c: &mut Criterion) {
    criterion_file(c, shared::GOLD_FILE, "gold file");
}

#[inline(always)]
fn criterion_file(c: &mut Criterion, file: &str, id: &str) {
    let content = fs::read(file).unwrap();
    c.bench_function(id, |b| b.iter(|| one_shot(black_box(&content))));
}

criterion_group!(benches, criterion_startup, criterion_short, criterion_gold);
criterion_main!(benches);
