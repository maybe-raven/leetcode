use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::max_profit::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8378205783205702;
    let dist = Uniform::new(1, 10000);
    let fee_dist = Uniform::new(0, 10000);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(dist);

    c.bench_function("max_profit", |b| {
        b.iter_batched(
            || {
                (
                    rng_iter.by_ref().take(rng.sample(dist) as usize).collect(),
                    rng.sample(fee_dist),
                )
            },
            |(prices, fee)| Solution::max_profit(prices, fee),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
