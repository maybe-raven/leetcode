use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetrust::tallest_billboard::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_setup(c: &mut Criterion) {
    const RAND_SEED: u64 = 1847812804610510;
    const RAND_SEED_1: u64 = 8378205783205702;
    let dist = Uniform::new_inclusive(1, 1000);
    let len_dist = Uniform::new_inclusive(1, 20);
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut rng_iter = StdRng::seed_from_u64(RAND_SEED_1).sample_iter(dist);

    c.bench_function("tallest_billboard", |b| {
        b.iter_batched(
            || {
                let mut rods: Vec<_> = rng_iter.by_ref().take(rng.sample(len_dist)).collect();

                loop {
                    rods.sort_unstable();

                    let mut n = 0;
                    let mut sum;
                    while {
                        sum = rods.iter().sum::<i32>();
                        sum > 5000 || (sum == 5000 && n != 0)
                    } {
                        rods.pop();
                        n += 1;
                    }

                    if n == 0 {
                        break;
                    }

                    let dist = Uniform::new_inclusive(1, 5000 - sum);
                    for _ in 0..n {
                        rods.push(rng.sample(dist));
                    }
                }

                rods
            },
            |rods| Solution::tallest_billboard(rods),
            criterion::BatchSize::SmallInput,
        )
    });
}

#[allow(dead_code)]
fn benchmark_fixed(c: &mut Criterion) {
    c.bench_function("tallest_billboard", |b| {
        b.iter(|| {
            Solution::tallest_billboard(black_box(vec![
                102, 101, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100,
            ]))
        })
    });
}

criterion_group!(benches, benchmark_setup);
criterion_main!(benches);
