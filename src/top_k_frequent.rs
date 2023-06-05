//! 347. Top K Frequent Elements
//! https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        vec.into_iter().take(k as usize).map(|x| x.0).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
