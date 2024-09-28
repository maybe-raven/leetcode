use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::num_of_ways::Solution;
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    let mut rng = StdRng::seed_from_u64(RAND_SEED);

    c.bench_function("num_of_ways", |b| {
        b.iter_batched(
            || (-750..=750).choose_multiple(&mut rng, 1000),
            |nums| Solution::num_of_ways(nums),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
