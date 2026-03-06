use criterion::{Criterion, criterion_group, criterion_main};
use optimization_practice::{process_json, sum_numbers};
use std::hint::black_box;

fn bench_process_json(c: &mut Criterion) {
    let data = (0..10000)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let json_data = format!(r#"[{}]"#, data);

    c.bench_function("process_json", |b| {
        b.iter(|| {
            let numbers = process_json(black_box(&json_data)).unwrap();
            black_box(sum_numbers(&numbers));
        });
    });
}

criterion_group!(benches, bench_process_json);
criterion_main!(benches);
