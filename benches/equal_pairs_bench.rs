use std::iter::repeat_with;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetrust::equal_pairs::Solution;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

fn benchmark_fun(c: &mut Criterion) {
    const RAND_SEED: u64 = 591248185340547350;
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let dist = Uniform::new(1, 10);

    let mut group = c.benchmark_group("solve_sudoku");

    for n in [10, 50, 150, 200] {
        let grid: Vec<Vec<i32>> = repeat_with(|| (&mut rng).sample_iter(dist).take(n).collect())
            .take(n)
            .collect();

        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_batched(
                || grid.clone(),
                |grid| Solution::equal_pairs(grid),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
