//! 1351. Count Negative Numbers in a Sorted Matrix
//! https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix

// Starting from the top right or the bottom left,
// moving left or moving up is increasing,
// moving right or moving down is decreasing.

// Start from top right, move left till the first non-negative number,
// move down by one, move left till the first non-negative number,
// repeat

const EMPTY_INPUT_ERR: &'static str = "Input must not be empty";

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.first().expect(EMPTY_INPUT_ERR).len();
        let mut i = m - 1;
        let mut count = 0;

        for row in grid.iter() {
            while i > 0 && row[i] < 0 {
                i -= 1;
            }

            if row[i] < 0 {
                count += m - i;
            } else {
                count += m - i - 1;
            }
        }

        count as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negative() {
        assert_eq!(
            8,
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3],
            ])
        );
        assert_eq!(0, Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]));
        assert_eq!(0, Solution::count_negatives(vec![vec![0]]));
        assert_eq!(1, Solution::count_negatives(vec![vec![-1]]));
    }
}
