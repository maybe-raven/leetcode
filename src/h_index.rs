//! https://leetcode.com/problems/h-index/

use std::convert::TryInto;

fn usize_to_i32(i: usize) -> i32 {
    i.try_into().unwrap()
}

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|a, b| b.cmp(a));

        if let Some(result) = citations
            .iter()
            .enumerate()
            .position(|(i, &c)| usize_to_i32(i + 1) > c)
        {
            usize_to_i32(result)
        } else {
            usize_to_i32(citations.len())
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![4, 4, 0, 0]), 2);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
        assert_eq!(Solution::h_index(vec![11, 15]), 2);
        assert_eq!(Solution::h_index(vec![100]), 1);
    }
}
