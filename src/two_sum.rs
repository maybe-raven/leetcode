//! 1. Two Sum
//! https://leetcode.com/problems/two-sum

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut memo = HashMap::new();

        for (index, num) in nums.into_iter().enumerate() {
            if let Some(&i) = memo.get(&num) {
                return vec![index as i32, i as i32];
            }

            memo.insert(target - num, index);
        }

        panic!("Input is assumed to have a solution.");
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert!({
            let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
            result.contains(&0) && result.contains(&1)
        });
        assert!({
            let result = Solution::two_sum(vec![3, 2, 4], 6);
            result.contains(&2) && result.contains(&1)
        });
        assert!({
            let result = Solution::two_sum(vec![3, 3], 6);
            result.contains(&0) && result.contains(&1)
        });
    }
}
