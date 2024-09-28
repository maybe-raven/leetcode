use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetrust::solve_sudoku::{Solution, SudokuMaker};
use rand::{rngs::StdRng, SeedableRng};

fn benchmark_fun(c: &mut Criterion) {
    const RAND_SEED: u64 = 591248185340547350;
    let mut maker = SudokuMaker::new(StdRng::seed_from_u64(RAND_SEED));
    let mut group = c.benchmark_group("solve_sudoku");

    for n in [17, 34, 51, 68] {
        let board = maker.make_sudoku_puzzle(n);

        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_batched(
                || board.clone(),
                |mut board| Solution::solve_sudoku(&mut board),
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_fun);
criterion_main!(benches);
