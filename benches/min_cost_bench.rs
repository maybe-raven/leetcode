use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::min_cost::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8378205783205702;
    let value_dist = Uniform::new(1, 1000000);
    let len_dist = Uniform::new(1000, 100000);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(value_dist);

    c.bench_function("min_cost", |b| {
        b.iter_batched(
            || {
                let len = rng.sample(len_dist);
                (
                    rng_iter.by_ref().take(len).collect(),
                    rng_iter.by_ref().take(len).collect(),
                )
            },
            |(nums, costs)| Solution::min_cost(nums, costs),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
