use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roger::one_shot;
use std::fs;

const GOLD_FILE: &str = "fuzz/corpus/roger/e6ca62714f91b0b0f70ca5681c146e2d3f3aa8ff";
const STARTUP_FILE: &str = "fuzz/corpus/roger/fa6435a685f664c07212d0eb5ebc1366bdfd5080";

pub fn criterion_gold(c: &mut Criterion) {
    criterion_file(c, GOLD_FILE, "gold file");
}

pub fn criterion_startup(c: &mut Criterion) {
    criterion_file(c, STARTUP_FILE, "startup file");
}

fn criterion_file(c: &mut Criterion, file: &str, id: &str) {
    let content = fs::read(file).unwrap();
    c.bench_function(id, |b| b.iter(|| one_shot(black_box(&content))));
}

criterion_group!(benches, criterion_gold, criterion_startup);
criterion_main!(benches);
