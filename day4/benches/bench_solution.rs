use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let rows = day4::load("test_input.txt");
    c.bench_function("p1 solution", |b| {
        b.iter(|| day4::p1(black_box(rows.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
