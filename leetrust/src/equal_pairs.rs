//! 2352. Equal Row and Column Pairs
//! https://leetcode.com/problems/equal-row-and-column-pairs

use std::collections::BTreeMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut counter: BTreeMap<&Vec<i32>, usize> = BTreeMap::new();

        grid.iter().for_each(|row| {
            counter
                .entry(row)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let mut key = grid[0].clone();

        (0..grid.len())
            .filter_map(|column| {
                key.iter_mut()
                    .zip(grid.iter())
                    .for_each(|(a, b)| *a = b[column]);
                counter.get(&key)
            })
            .sum::<usize>() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            1,
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]])
        );
        assert_eq!(
            3,
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ])
        );
        assert_eq!(
            4,
            Solution::equal_pairs(vec![
                vec![1, 2, 2, 4, 5],
                vec![2, 4, 4, 5, 6],
                vec![3, 4, 4, 6, 7],
                vec![2, 4, 4, 4, 4],
                vec![2, 4, 4, 4, 4]
            ])
        );
    }
}
