use criterion::{criterion_group, criterion_main, Criterion};
use day12::step1;
use std::fs;

fn benchmark(c: &mut Criterion) {
    let test = fs::read_to_string("test.txt").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    {
        let mut group = c.benchmark_group("rayon_test");
        group.bench_function("step1_test", |b| b.iter(|| step1(&test, false)));
        group.bench_function("step1_test_par", |b| b.iter(|| step1(&test, true)));
    }
    {
        let mut group = c.benchmark_group("rayon_input");
        group.bench_function("step1_input", |b| b.iter(|| step1(&input, false)));
        group.bench_function("step1_input_par", |b| b.iter(|| step1(&input, true)));
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
