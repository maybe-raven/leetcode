use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(row: usize, column: usize) -> Coordinate {
        Self { row, column }
    }

    fn to_block_start(self) -> Coordinate {
        Self {
            row: self.row - self.row % 3,
            column: self.column - self.column % 3,
        }
    }

    const BLOCK_INDICES: [Self; 9] = [
        Coordinate { row: 0, column: 0 },
        Coordinate { row: 0, column: 3 },
        Coordinate { row: 0, column: 6 },
        Coordinate { row: 3, column: 0 },
        Coordinate { row: 3, column: 3 },
        Coordinate { row: 3, column: 6 },
        Coordinate { row: 6, column: 0 },
        Coordinate { row: 6, column: 3 },
        Coordinate { row: 6, column: 6 },
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Tile(u8);

impl Tile {
    const EMPTY_VALUE: Tile = Tile(b'.');
    const ALL_VALUES: [Tile; 9] = [
        Tile(b'1'),
        Tile(b'2'),
        Tile(b'3'),
        Tile(b'4'),
        Tile(b'5'),
        Tile(b'6'),
        Tile(b'7'),
        Tile(b'8'),
        Tile(b'9'),
    ];

    fn is_empty(self) -> bool {
        self == Self::EMPTY_VALUE
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self(value as u8)
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        value.0 as char
    }
}

struct Board([[Tile; 9]; 9]);

impl TryFrom<&Vec<Vec<char>>> for Board {
    type Error = ();

    fn try_from(char_board: &Vec<Vec<char>>) -> Result<Self, Self::Error> {
        if char_board.len() < 9 {
            return Err(());
        }

        let mut arr_board = [[Tile::default(); 9]; 9];
        for (arr_row, char_row) in arr_board.iter_mut().zip(char_board) {
            if char_row.len() < 9 {
                return Err(());
            }

            for (arr_tile, &char_tile) in arr_row.iter_mut().zip(char_row) {
                *arr_tile = Tile::from(char_tile);
            }
        }

        Ok(Self(arr_board))
    }
}

impl Board {
    fn new() -> Self {
        Self([[Tile::EMPTY_VALUE; 9]; 9])
    }
    fn find_first_empty(&mut self) -> Option<Coordinate> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile.is_empty() {
                    return Some(Coordinate::new(y, x));
                }
            }
        }
        None
    }

    fn get_column(&self, column: usize) -> [Tile; 9] {
        [
            self.0[0][column],
            self.0[1][column],
            self.0[2][column],
            self.0[3][column],
            self.0[4][column],
            self.0[5][column],
            self.0[6][column],
            self.0[7][column],
            self.0[8][column],
        ]
    }

    fn get_columns(&self) -> impl Iterator<Item = [Tile; 9]> + '_ {
        (0..9).map(|column| self.get_column(column))
    }

    fn get_block(&self, Coordinate { row, column }: Coordinate) -> [Tile; 9] {
        [
            self.0[row][column],
            self.0[row][column + 1],
            self.0[row][column + 2],
            self.0[row + 1][column],
            self.0[row + 1][column + 1],
            self.0[row + 1][column + 2],
            self.0[row + 2][column],
            self.0[row + 2][column + 1],
            self.0[row + 2][column + 2],
        ]
    }

    fn get_possible_values(&self, coord: Coordinate) -> [Tile; 9] {
        let mut values = Tile::ALL_VALUES;

        let mut clear_value = |tile: Tile| {
            let Some(i) = char::to_digit(tile.0 as char, 10) else { return; };
            values[(i - 1) as usize] = Tile::EMPTY_VALUE;
        };

        self.0[coord.row].iter().copied().for_each(&mut clear_value);
        self.0
            .iter()
            .map(|row| row[coord.column])
            .for_each(&mut clear_value);

        let Coordinate {
            row: block_row,
            column: block_column,
        } = coord.to_block_start();

        self.0[block_row..block_row + 3]
            .iter()
            .flat_map(|row| row[block_column..block_column + 3].iter())
            .copied()
            .for_each(clear_value);

        values
    }

    fn is_solved(&self) -> bool {
        fn check_group(group: &[Tile; 9]) -> bool {
            Tile::ALL_VALUES.iter().all(|v| group.contains(v))
        }

        if !self.0.iter().all(check_group) {
            return false;
        }

        if !self.get_columns().all(|col| check_group(&col)) {
            return false;
        }

        Coordinate::BLOCK_INDICES
            .iter()
            .map(|&coord| self.get_block(coord))
            .all(|block| check_group(&block))
    }

    fn solve(&mut self) -> bool {
        let Some(coord) = self.find_first_empty() else { return self.is_solved(); };

        let result = self
            .get_possible_values(coord)
            .into_iter()
            .filter(|x| !x.is_empty())
            .any(|value| {
                self[coord] = value;
                self.solve()
            });

        if result {
            true
        } else {
            self[coord] = Tile::EMPTY_VALUE;
            false
        }
    }

    fn sync(&self, out: &mut [Vec<char>]) {
        for (out_row, row) in out.iter_mut().zip(self.0.iter()) {
            for (out_tile, &tile) in out_row.iter_mut().zip(row.iter()) {
                *out_tile = tile.into();
            }
        }
    }
}

impl Index<Coordinate> for Board {
    type Output = Tile;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.0.index(index.row).index(index.column)
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.0.index_mut(index.row).index_mut(index.column)
    }
}

impl Solution {
    pub fn solve_sudoku(og_board: &mut Vec<Vec<char>>) {
        let mut board = Board::try_from(&*og_board).unwrap();
        board.solve();
        board.sync(og_board);
    }
}

pub struct Solution;

pub struct SudokuMaker<R> {
    rng: R,
}

impl<R: Rng> SudokuMaker<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    pub fn make_sudoku_puzzle(&mut self, n: usize) -> Vec<Vec<char>> {
        let all_coordinates: Vec<Coordinate> = (0..9)
            .flat_map(|x| (0..9).map(move |y| Coordinate::new(y, x)))
            .collect();
        let mut board = Board::new();

        for (i, &coord) in all_coordinates
            .choose_multiple(&mut self.rng, 9)
            .enumerate()
        {
            board[coord] = (i + 1).to_string().chars().next().unwrap().into();
        }

        board.solve();

        for &coord in all_coordinates.choose_multiple(&mut self.rng, 81 - n) {
            board[coord] = Tile::EMPTY_VALUE;
        }

        board
            .0
            .into_iter()
            .map(|row| row.into_iter().map(char::from).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            [
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );

        let mut board = vec![
            vec!['1', '.', '.', '4', '.', '.', '.', '.', '.'],
            vec!['.', '5', '.', '.', '8', '.', '.', '.', '.'],
            vec!['.', '.', '9', '1', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '.', '.', '7'],
            vec!['.', '.', '8', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '5', '.', '.', '4', '.', '.', '.'],
            vec!['.', '.', '.', '8', '.', '.', '.', '.', '.'],
            vec!['6', '.', '1', '.', '4', '2', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '.', '.', '.'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            [
                ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
                ['4', '5', '6', '7', '8', '9', '1', '2', '3'],
                ['7', '8', '9', '1', '2', '3', '4', '5', '6'],
                ['2', '6', '4', '3', '1', '8', '5', '9', '7'],
                ['3', '7', '8', '6', '9', '5', '2', '1', '4'],
                ['9', '1', '5', '2', '7', '4', '3', '6', '8'],
                ['5', '4', '2', '8', '6', '7', '9', '3', '1'],
                ['6', '3', '1', '9', '4', '2', '8', '7', '5'],
                ['8', '9', '7', '5', '3', '1', '6', '4', '2']
            ]
        );

        let mut board = vec![
            vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '6', '.', '.', '.', '.', '.'],
            vec!['.', '7', '.', '.', '9', '.', '2', '.', '.'],
            vec!['.', '5', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '.', '4', '5', '7', '.', '.'],
            vec!['.', '.', '.', '1', '.', '.', '.', '3', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '6', '8'],
            vec!['.', '.', '8', '5', '.', '.', '.', '1', '.'],
            vec!['.', '9', '.', '.', '.', '.', '4', '.', '.'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            [
                ['8', '1', '2', '7', '5', '3', '6', '4', '9'],
                ['9', '4', '3', '6', '8', '2', '1', '7', '5'],
                ['6', '7', '5', '4', '9', '1', '2', '8', '3'],
                ['1', '5', '4', '2', '3', '7', '8', '9', '6'],
                ['3', '6', '9', '8', '4', '5', '7', '2', '1'],
                ['2', '8', '7', '1', '6', '9', '5', '3', '4'],
                ['5', '2', '1', '9', '7', '4', '3', '6', '8'],
                ['4', '3', '8', '5', '2', '6', '9', '1', '7'],
                ['7', '9', '6', '3', '1', '8', '4', '5', '2']
            ]
        );
    }
}
