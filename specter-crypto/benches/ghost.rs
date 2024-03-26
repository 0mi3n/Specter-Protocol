//! Defines benchmarks for the Ghost hash function

#![allow(missing_docs)]

use constants::Scalar;
use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use rand::thread_rng;

/// Run a benchmark on the Ghost 2 hash implementation
fn bench_Ghost2(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut group = c.benchmark_group("Ghost2-hash");
    for i in [1, 10, 100, 1000] {
        group.throughput(Throughput::Elements(i));
        group.bench_function(BenchmarkId::from_parameter(i), |b| {
            b.iter_batched(
                || (0..i).map(|_| Scalar::random(&mut rng)).collect::<Vec<_>>(),
                |input| {
                    black_box(compute_Ghost_hash(&input));
                },
                BatchSize::SmallInput,
            );
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_Ghost2
);
criterion_main!(benches);
