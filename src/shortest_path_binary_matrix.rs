//! https://leetcode.com/problems/shortest-path-in-binary-matrix/

use std::{cmp::min, collections::VecDeque, ops::IndexMut};

fn safe_decrement(value: usize) -> usize {
    if value == 0 {
        value
    } else {
        value - 1
    }
}

fn check_and_push(
    grid: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
    path_length: i32,
    queue: &mut VecDeque<(usize, usize)>,
) {
    let tile = grid.index_mut(y).index_mut(x);
    if *tile == 0 {
        *tile = path_length;
        queue.push_back((x, y));
    }
}

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() - 1;

        if grid[0][0] == 1 || grid[n][n] == 1 {
            return -1;
        }

        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 2;
        }

        let mut queue = VecDeque::new();

        check_and_push(&mut grid, n - 1, n, 2, &mut queue);
        check_and_push(&mut grid, n, n - 1, 2, &mut queue);
        check_and_push(&mut grid, n - 1, n - 1, 2, &mut queue);

        let mut tile;
        let mut path_length;

        while let Some((x, y)) = queue.pop_front() {
            tile = grid[y][x];

            if x == 0 && y == 0 {
                return tile;
            }

            path_length = tile + 1;

            for neighbor_x in safe_decrement(x)..=min(n, x + 1) {
                for neighbor_y in safe_decrement(y)..=min(n, y + 1) {
                    if x == neighbor_x && y == neighbor_y {
                        continue;
                    }

                    check_and_push(&mut grid, neighbor_x, neighbor_y, path_length, &mut queue);
                }
            }
        }

        -1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0]]), 1);
        assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![1]]), -1);
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 0],
            ]),
            8
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 1, 1, 0, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 1, 1, 0],
            ]),
            13
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
            ]),
            -1
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 1, 0, 1, 1],
                vec![1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0]
            ]),
            6
        );
    }
}
