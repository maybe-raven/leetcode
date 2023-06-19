use std::iter::repeat_with;

use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::make_array_increasing::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let len_dist = Uniform::new(1000, 2000);
    let value_dist = Uniform::new(0, 1000000000);

    c.bench_function("num_of_ways", |b| {
        b.iter_batched(
            || {
                let n = rng.sample(len_dist);
                let m = rng.sample(len_dist);

                (
                    repeat_with(|| rng.sample(value_dist)).take(n).collect(),
                    repeat_with(|| rng.sample(value_dist)).take(m).collect(),
                )
            },
            |(arr1, arr2)| Solution::make_array_increasing(arr1, arr2),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_with_fixed_data(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_ELECTRIC_BOOGALOO: u64 = 7467834673459637;
    let len_dist = Uniform::new(1000, 2000);
    let value_dist = Uniform::new(0, 1000000000);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED).sample_iter(value_dist);
    let mut rng = StdRng::seed_from_u64(RAND_SEED_ELECTRIC_BOOGALOO);
    let arr1: Vec<i32> = rng_iter.by_ref().take(rng.sample(len_dist)).collect();
    let arr2: Vec<i32> = rng_iter.by_ref().take(rng.sample(len_dist)).collect();

    c.bench_function("num_of_ways", |b| {
        b.iter_batched(
            || (arr1.clone(), arr2.clone()),
            |(arr1, arr2)| Solution::make_array_increasing(arr1, arr2),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_with_fixed_data);
criterion_main!(benches);
