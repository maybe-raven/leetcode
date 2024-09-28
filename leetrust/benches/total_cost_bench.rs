use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::total_cost::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8378205783205702;
    let dist = Uniform::new_inclusive(1, 100000);
    let len_dist = Uniform::new_inclusive(1000, 100000);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(dist);

    c.bench_function("total_cost", |b| {
        b.iter_batched(
            || {
                let len = rng.sample(len_dist);
                let k_dist = Uniform::new_inclusive(1, len as i32);
                (
                    rng_iter.by_ref().take(len).collect(),
                    rng.sample(k_dist),
                    rng.sample(k_dist),
                )
            },
            |(costs, k, cand)| Solution::total_cost(costs, k, cand),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
