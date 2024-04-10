use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roger::one_shot;
use std::fs;

const GOLD_FILE: &str = "fuzz/corpus/roger/eaaaac643234c7f28b0c93b9b80bf87ff43ee6cb";

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = fs::read_to_string(GOLD_FILE).unwrap().as_bytes().to_owned();
    c.bench_function("gold", |b| b.iter(|| one_shot(black_box(&content))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
