//! 2462. Total Cost to Hire K Workers
//! https://leetcode.com/problems/total-cost-to-hire-k-workers

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        assert!(1 <= k && k as usize <= costs.len());
        assert!(1 <= candidates && candidates as usize <= costs.len());
        assert!(1 <= costs.len());

        let k = k as usize;
        let n = candidates as usize;

        let mut left_cursor = n;
        let mut right_cursor = costs.len() - n;

        if right_cursor < left_cursor + k {
            let mut costs = costs;
            costs.sort_unstable();
            return costs.into_iter().take(k).map(|x| x as i64).sum();
        }

        let mut left_heap: BinaryHeap<Reverse<i32>> =
            costs[..left_cursor].iter().copied().map(Reverse).collect();
        let mut right_heap: BinaryHeap<Reverse<i32>> =
            costs[right_cursor..].iter().copied().map(Reverse).collect();

        right_cursor -= 1;

        let Reverse(mut left) = left_heap.pop().unwrap();
        let Reverse(mut right) = right_heap.pop().unwrap();

        let mut total = 0;

        for _ in 0..k {
            if right < left {
                total += right as i64;
                right_heap.push(Reverse(costs[right_cursor]));
                right_cursor -= 1;
                Reverse(right) = right_heap.pop().unwrap();
            } else {
                total += left as i64;
                left_heap.push(Reverse(costs[left_cursor]));
                left_cursor += 1;
                Reverse(left) = left_heap.pop().unwrap();
            }
        }

        total
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            11,
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
        );
        assert_eq!(4, Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
        assert_eq!(4, Solution::total_cost(vec![3, 2, 1, 1, 7, 2], 3, 2));
        assert_eq!(
            106,
            Solution::total_cost(
                vec![100, 99, 98, 97, 1, 2, 3, 4, 5, 6, 7, 8, 80, 81, 82, 83],
                5,
                4
            )
        );
    }
}
