use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::splish;
use testing::sploosh;

pub fn splish_benchmark(c: &mut Criterion) {
    c.bench_function("splish", |b| b.iter(|| splish(black_box(2), black_box(3))));
}

pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh", |b| {
        b.iter(|| sploosh(black_box(1), black_box(2), black_box(3)))
    });
}

criterion_group!(benches, splish_benchmark, sploosh_benchmark);
criterion_main!(benches);
