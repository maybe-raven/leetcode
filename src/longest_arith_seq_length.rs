//! 1027. Longest Arithmetic Subsequence
//! https://leetcode.com/problems/longest-arithmetic-subsequence

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;

        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().enumerate().skip(i + 1) {
                let step = a - b;
                let mut count = 2;
                let mut expected = b - step;
                for &c in &nums[j + 1..] {
                    if c == expected {
                        count += 1;
                        expected -= step;
                    }
                }
                max_count = max_count.max(count);
            }
        }

        max_count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(4, Solution::longest_arith_seq_length(vec![3, 6, 9, 12]));
        assert_eq!(3, Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]));
        assert_eq!(
            4,
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8])
        );
    }
}
