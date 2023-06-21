//! 2448. Minimum Cost to Make Array Equal
//! https://leetcode.com/problems/minimum-cost-to-make-array-equal

use std::{cmp::min, iter::once};

impl Solution {
    pub fn min_cost(nums: Vec<i32>, costs: Vec<i32>) -> i64 {
        assert_eq!(nums.len(), costs.len());
        if nums.len() < 2 {
            return 0;
        }

        let nums: Vec<(i64, i64)> = {
            let mut nums: Vec<(i64, i64)> = nums
                .into_iter()
                .zip(costs)
                .map(|(a, b)| (a as i64, b as i64))
                .chain(once((0, 0)))
                .collect();
            nums.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            nums
        };

        let mut sum = nums.iter().map(|(num, cost)| num * cost).sum();
        let mut base = -nums.iter().map(|(_, cost)| cost).sum::<i64>();

        let mut min_total_cost = i64::MAX;

        for window in nums.windows(2).rev() {
            let &[(b, cost), (a, _a_cost)] = window else { unreachable!() };

            let diff = b - a;
            sum += base * diff;
            min_total_cost = min(min_total_cost, sum);
            base += cost * 2;
        }

        min_total_cost
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(0, Solution::min_cost(vec![1000000], vec![1000000]));
        assert_eq!(8, Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]));
        // [1,2,3,5] <= [1, 3, 1, 2]
        // [0,1,2,4]
        //  ^
        // [1,0,1,3]
        //    ^
        // [2,1,0,2]
        //      ^
        // [4,3,2,0]
        //        ^
        // [1,1,3,0]
        // [14, 3, 2, 1] <= [3, 1, 0, 2]
        assert_eq!(8, Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 4]));
        assert_eq!(10, Solution::min_cost(vec![1, 3, 5, 2], vec![2, 2, 2, 3]));
        assert_eq!(8, Solution::min_cost(vec![1, 1, 1, 9], vec![9, 1, 1, 1]));
        assert_eq!(72, Solution::min_cost(vec![1, 1, 1, 9], vec![8, 8, 8, 9]));
        assert_eq!(
            0,
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3])
        );
    }
}
