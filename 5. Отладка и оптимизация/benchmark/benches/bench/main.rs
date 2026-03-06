use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn sum_vec(items: &[u32]) -> u64 {
    items.iter().map(|&x| x as u64).sum()
}

fn bench_sum(c: &mut Criterion) {
    let data: Vec<u32> = (0..1000).collect();

    c.bench_function("sum_1000", |b| {
        b.iter(|| sum_vec(black_box(&data)));
    });
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
