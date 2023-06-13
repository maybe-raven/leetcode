use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetrust::solve_sudoku::{Board, Cell, Coordinate, Solution};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

const RAND_SEED: u64 = 591248185340547350;

// pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
//     let mut grid = Grid::from(&*board);
//     grid.solve();
//     *board = grid.into();
// }

fn benchmark_fun(c: &mut Criterion) {
    let all_coordinates: Vec<Coordinate> = (0..9)
        .flat_map(|x| (0..9).map(move |y| Coordinate::new(y, x)))
        .collect();
    let mut rng = StdRng::seed_from_u64(RAND_SEED);
    let mut group = c.benchmark_group("solve_sudoku");

    for n in [17, 34, 51, 68] {
        let board: Vec<Vec<char>> = {
            let mut og_board = vec![vec!['.'; 9]; 9];
            let mut board = Board::try_from(&og_board).unwrap();

            for (i, &coord) in all_coordinates.choose_multiple(&mut rng, 9).enumerate() {
                board[coord] = (i + 1).to_string().chars().next().unwrap().into();
            }

            board.solve();

            for &coord in all_coordinates.choose_multiple(&mut rng, 81 - n) {
                board[coord] = Cell::from('.');
            }

            board.sync(&mut og_board);
            og_board
        };

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
