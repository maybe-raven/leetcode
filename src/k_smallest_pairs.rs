//! 373. Find K Pairs with Smallest Sums
//! https://leetcode.com/problems/find-k-pairs-with-smallest-sums

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    ops::Add,
};

#[derive(Debug)]
struct Item<T, I> {
    sum: T,
    head: (T, T),
    source: I,
}

impl<T: Add<T, Output = T> + Copy, I: Iterator<Item = (T, T)>> Item<T, I> {
    fn new(mut source: I) -> Option<Self> {
        if let Some(head) = source.next() {
            Some(Self {
                sum: head.0 + head.1,
                head,
                source,
            })
        } else {
            None
        }
    }

    fn next(&mut self) -> bool {
        if let Some(head) = self.source.next() {
            self.sum = head.0 + head.1;
            self.head = head;
            true
        } else {
            false
        }
    }
}

impl<T: Eq, I> Eq for Item<T, I> {}
impl<T: PartialEq, I> PartialEq for Item<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.sum.eq(&other.sum)
    }
}

impl<T: PartialOrd, I> PartialOrd for Item<T, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sum.partial_cmp(&other.sum)
    }
}

impl<T: Ord, I> Ord for Item<T, I> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum.cmp(&other.sum)
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: BinaryHeap<_> = nums1
            .iter()
            .filter_map(|&x| Item::new(nums2.iter().map(move |&y| (x, y))).map(|x| Reverse(x)))
            .collect();

        let mut results = Vec::with_capacity(k);

        for _ in 0..k {
            let Some(Reverse(mut item)) = heap.pop() else { return results; };
            let (a, b) = item.head;
            results.push(vec![a, b]);
            if item.next() {
                heap.push(Reverse(item));
            }
        }

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_shit(expected: Vec<Vec<i32>>, output: Vec<Vec<i32>>) {
        assert_eq!(expected.len(), output.len());

        for pair in &output {
            assert_eq!(2, pair.len());
        }

        for pair in expected {
            {
                assert!(
                    output.iter().any(|x| pair.iter().all(|y| x.contains(y))),
                    "{:?} doesn't contain {:?}",
                    &output,
                    &pair
                );
            }
        }
    }

    #[test]
    fn test_solution() {
        assert_shit(
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        );
        assert_shit(
            vec![vec![1, 1], vec![1, 1]],
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        );
        assert_shit(
            vec![vec![1, 3], vec![2, 3]],
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
        );
        assert_shit(
            vec![vec![1, 3], vec![2, 3], vec![3, 3], vec![4, 3]],
            Solution::k_smallest_pairs(vec![1, 2, 3, 4, 5, 6], vec![3], 4),
        );
        assert_shit(
            vec![
                vec![1, 1],
                vec![1, 1],
                vec![2, 1],
                vec![1, 2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 3],
                vec![1, 3],
                vec![2, 3],
            ],
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 10),
        );
        assert_shit(
            vec![
                vec![1, 2],
                vec![1, 4],
                vec![3, 2],
                vec![1, 6],
                vec![3, 4],
                vec![5, 2],
                vec![1, 8],
                vec![3, 6],
                vec![5, 4],
                vec![7, 2],
            ],
            Solution::k_smallest_pairs(vec![1, 3, 5, 7], vec![2, 4, 6, 8], 10),
        );
    }
}
