use std::{
    collections::BTreeSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    pub fn new(row: usize, column: usize) -> Coordinate {
        Self { row, column }
    }

    fn to_block_start(&self) -> Coordinate {
        Self {
            row: self.row - self.row % 3,
            column: self.column - self.column % 3,
        }
    }

    pub const BLOCK_INDICES: [Self; 9] = [
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
pub struct Cell(u8);

impl Cell {
    const EMPTY_VALUE: Cell = Cell(b'.');
    const ALL_VALUES: [Cell; 9] = [
        Cell(b'1'),
        Cell(b'2'),
        Cell(b'3'),
        Cell(b'4'),
        Cell(b'5'),
        Cell(b'6'),
        Cell(b'7'),
        Cell(b'8'),
        Cell(b'9'),
    ];

    fn is_empty(self) -> bool {
        self == Self::EMPTY_VALUE
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self(value as u8)
    }
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        value.0 as char
    }
}

pub struct Board([[Cell; 9]; 9]);

// impl From<&Vec<Vec<char>>> for Board {
//     fn from(value: &Vec<Vec<char>>) -> Self {
//         Self(
//             value
//                 .into_iter()
//                 .map(|row| row.into_iter().map(|&cell| cell.into()).collect())
//                 .collect(),
//         )
//     }
// }

impl TryFrom<&Vec<Vec<char>>> for Board {
    type Error = ();

    fn try_from(char_board: &Vec<Vec<char>>) -> Result<Self, Self::Error> {
        if char_board.len() < 9 {
            return Err(());
        }

        let mut arr_board = [[Cell::default(); 9]; 9];
        for (arr_row, char_row) in arr_board.iter_mut().zip(char_board) {
            if char_row.len() < 9 {
                return Err(());
            }

            for (arr_cell, &char_cell) in arr_row.iter_mut().zip(char_row) {
                *arr_cell = Cell::from(char_cell);
            }
        }

        Ok(Self(arr_board))
    }
}

impl Board {
    // pub fn new(board: Vec<Vec<Cell>>) -> Self {
    //     Self(board)
    // }

    fn find_first_empty(&mut self) -> Option<Coordinate> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell.is_empty() {
                    return Some(Coordinate::new(y, x));
                }
            }
        }
        None
    }

    fn get_column(&self, column: usize) -> [Cell; 9] {
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

    fn get_columns(&self) -> impl Iterator<Item = [Cell; 9]> + '_ {
        (0..9).map(|column| self.get_column(column))
    }

    fn get_block(&self, coord: Coordinate) -> [Cell; 9] {
        [
            self.0[coord.row][coord.column],
            self.0[coord.row][coord.column + 1],
            self.0[coord.row][coord.column + 2],
            self.0[coord.row + 1][coord.column],
            self.0[coord.row + 1][coord.column + 1],
            self.0[coord.row + 1][coord.column + 2],
            self.0[coord.row + 2][coord.column],
            self.0[coord.row + 2][coord.column + 1],
            self.0[coord.row + 2][coord.column + 2],
        ]
    }

    fn get_connected(&self, coordinate: Coordinate) -> [Cell; 25] {
        let mut connected = [Cell::default(); 25];
        connected[..9].copy_from_slice(&self.0[coordinate.row]);
        connected.swap(coordinate.column, 8);

        let mut column = self.get_column(coordinate.column);
        column.swap(coordinate.row, 8);
        connected[8..17].copy_from_slice(&column);

        connected[16..].copy_from_slice(&self.get_block(coordinate.to_block_start()));

        connected
    }

    pub fn get_possible_values(&self, index: Coordinate) -> impl Iterator<Item = Cell> {
        let existing_values: BTreeSet<Cell> = self
            .get_connected(index)
            .into_iter()
            .filter(|&c| !c.is_empty())
            .collect();

        Cell::ALL_VALUES
            .clone()
            .into_iter()
            .filter(move |x| !existing_values.contains(x))
    }

    fn is_solved(&self) -> bool {
        fn check_segment(segment: &[Cell]) -> bool {
            Cell::ALL_VALUES
                .iter()
                .all(|value| segment.into_iter().any(|cell| cell.eq(value)))
        }

        if !self.0.iter().all(|row| check_segment(row.as_slice())) {
            return false;
        }

        if !self.get_columns().all(|col| check_segment(&col)) {
            return false;
        }

        Coordinate::BLOCK_INDICES
            .iter()
            .map(|&coord| self.get_block(coord))
            .all(|block| check_segment(&block))
    }

    pub fn solve(&mut self) -> bool {
        let Some(coord) = self.find_first_empty() else { return self.is_solved(); };

        let result = self.get_possible_values(coord).into_iter().find(|&value| {
            self[coord] = value;
            self.solve()
        });

        if result.is_some() {
            true
        } else {
            self[coord] = Cell::EMPTY_VALUE;
            false
        }
    }

    pub fn sync(&self, out: &mut Vec<Vec<char>>) {
        for (out_row, row) in out.iter_mut().zip(self.0.iter()) {
            for (out_cell, &cell) in out_row.iter_mut().zip(row.into_iter()) {
                *out_cell = cell.into();
            }
        }
    }
}

impl Index<Coordinate> for Board {
    type Output = Cell;

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
