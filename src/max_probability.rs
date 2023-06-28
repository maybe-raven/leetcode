//! 1514. Path with Maximum Probability
//! https://leetcode.com/problems/path-with-maximum-probability

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Copy)]
struct Item {
    node: usize,
    probability: f64,
}

impl Item {
    fn new(node: usize, probability: f64) -> Self {
        Self { node, probability }
    }
}

impl Eq for Item {}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.node.eq(&other.node) && self.probability.eq(&other.probability)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.probability.partial_cmp(&other.probability).unwrap() {
            Ordering::Equal => self.node.cmp(&other.node),
            x => x,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

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

        let mut memo = vec![0.0; n];
        memo[end] = 1.0;

        let mut queue = BinaryHeap::new();
        queue.push(Item::new(end, 1.0));

        while let Some(a) = queue.pop() {
            if a.node == start {
                return a.probability;
            }

            for &(b, mut prob) in &edge_map[a.node] {
                prob *= a.probability;

                if memo[b] < prob {
                    memo[b] = prob;
                    queue.push(Item::new(b, prob));
                }
            }
        }

        0.0
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
        assert_eq!(
            0.0,
            Solution::max_probability(
                6,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![4, 5]
                ],
                vec![0.5; 6],
                0,
                4
            )
        );
    }
}
