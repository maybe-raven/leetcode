//! 1514. Path with Maximum Probability
//! https://leetcode.com/problems/path-with-maximum-probability

use std::collections::VecDeque;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;

        let mut edge_map: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];

        for (edge, prob) in edges.into_iter().zip(succ_prob) {
            let a = edge[0] as usize;
            let b = edge[1] as usize;

            edge_map[a].push((b, prob));
            edge_map[b].push((a, prob));
        }

        let mut memo: Vec<f64> = vec![0.0; n];
        memo[end as usize] = 1.0;

        let mut queue = VecDeque::new();
        queue.push_back(end);

        while let Some(a) = queue.pop_front() {
            let memo_prob = memo[a as usize];

            for &(b, mut prob) in &edge_map[a] {
                prob *= memo_prob;

                let old_prob = memo.get_mut(b as usize).unwrap();
                if *old_prob < prob {
                    *old_prob = prob;
                    queue.push_back(b);
                }
            }
        }

        memo[start as usize]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            0.25,
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2,
            )
        );
        assert_eq!(
            0.3,
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.3],
                0,
                2,
            )
        );
        assert_eq!(
            0.0,
            Solution::max_probability(3, vec![vec![0, 1]], vec![0.5], 0, 2,)
        );
    }
}
