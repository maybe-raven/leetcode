//! 2462. Total Cost to Hire K Workers
//! https://leetcode.com/problems/total-cost-to-hire-k-workers

use std::{
    cmp::Ordering,
    ops::{Deref, IndexMut},
};

fn f<A, B: Copy, C: Deref<Target = Option<B>>>((a, c): (A, C)) -> Option<(A, B)> {
    if let Some(b) = *c {
        Some((a, b))
    } else {
        None
    }
}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        assert!(costs.len() >= k as usize);

        let candidates = candidates as usize;
        let mut costs: Vec<Option<i32>> = costs.into_iter().map(Some).collect();
        let mut total = 0;
        for _ in 0..k {
            let i = costs
                .iter()
                .enumerate()
                .filter_map(f)
                .take(candidates)
                .chain(
                    costs
                        .iter()
                        .enumerate()
                        .rev()
                        .filter_map(f)
                        .take(candidates),
                )
                .min_by(|(i, a), (j, b)| match a.cmp(b) {
                    Ordering::Equal => i.cmp(j),
                    ordering => ordering,
                })
                .expect("costs.len >= k")
                .0;
            let c = costs.index_mut(i);
            total += c.expect("`f` filters `None` values.") as i64;
            *c = None;
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
    }
}
