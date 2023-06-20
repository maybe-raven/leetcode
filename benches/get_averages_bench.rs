use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::get_averages::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8419847184018479;
    let dist = Uniform::new(0, 100000);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(dist);

    c.bench_function("get_averages", |b| {
        b.iter_batched(
            || {
                (
                    rng_iter.by_ref().take(rng.sample(dist) as usize).collect(),
                    rng.sample(dist),
                )
            },
            |(nums, k)| Solution::get_averages(nums, k),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
