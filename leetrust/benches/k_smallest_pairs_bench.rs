use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::k_smallest_pairs::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8378205783205702;
    let dist = Uniform::new_inclusive(-1000000000, 1000000000);
    let len_dist = Uniform::new_inclusive(10000, 100000);
    let k_dist = Uniform::new_inclusive(1000, 10000);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(dist);

    c.bench_function("k_smallest_pairs", |b| {
        b.iter_batched(
            || {
                let mut nums1: Vec<_> = rng_iter.by_ref().take(rng.sample(len_dist)).collect();
                let mut nums2: Vec<_> = rng_iter.by_ref().take(rng.sample(len_dist)).collect();

                nums1.sort_unstable();
                nums2.sort_unstable();

                (nums1, nums2, rng.sample(k_dist))
            },
            |(nums1, nums2, k)| Solution::k_smallest_pairs(nums1, nums2, k),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
