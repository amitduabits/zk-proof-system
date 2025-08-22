use criterion::{criterion_group, criterion_main, Criterion};

fn bench_example(c: &mut Criterion) {
    c.bench_function("example", |b| {
        b.iter(|| {
            std::hint::black_box(42);
        });
    });
}

criterion_group!(benches, bench_example);
criterion_main!(benches);
