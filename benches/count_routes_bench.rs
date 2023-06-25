use criterion::{criterion_group, criterion_main, Criterion};
use leetrust::count_routes::Solution;
use rand::{distributions::Uniform, rngs::StdRng, seq::IteratorRandom, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    let len_dist = Uniform::new_inclusive(60, 100);
    let fuel_dist = Uniform::new_inclusive(120, 200);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);

    c.bench_function("count_routes", |b| {
        b.iter_batched(
            || {
                let len = rng.sample(len_dist);
                let location_dist = Uniform::new(0, len as i32);
                (
                    (1..150).choose_multiple(&mut rng, len),
                    rng.sample(location_dist),
                    rng.sample(location_dist),
                    rng.sample(fuel_dist),
                )
            },
            |(locations, start, finish, fuel)| {
                Solution::count_routes(locations, start, finish, fuel)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
