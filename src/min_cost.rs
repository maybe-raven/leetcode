//! 2448. Minimum Cost to Make Array Equal
//! https://leetcode.com/problems/minimum-cost-to-make-array-equal

impl Solution {
    pub fn min_cost(nums: Vec<i32>, costs: Vec<i32>) -> i64 {
        let targets = {
            let mut targets = nums.clone();
            targets.sort_unstable();
            targets.dedup();
            targets
        };

        targets
            .into_iter()
            .map(|target| {
                nums.iter()
                    .zip(costs.iter())
                    .map(|(&num, &cost)| (num - target).abs() as i64 * cost as i64)
                    .sum()
            })
            .min()
            .unwrap_or(0)
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
