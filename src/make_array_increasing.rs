//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

use std::{
    cmp::{max, min},
    ops::Sub,
};

trait SetAll<T> {
    fn set_all(&mut self, value: T);
}

impl<T: Copy> SetAll<T> for [T] {
    fn set_all(&mut self, value: T) {
        for x in self {
            *x = value;
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Index {
    Inclusive(usize),
    Exclusive(usize),
}

impl Sub for Index {
    type Output = usize;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Index::Exclusive(lhs), Index::Inclusive(rhs))
            | (Index::Inclusive(lhs), Index::Inclusive(rhs)) => lhs.saturating_sub(rhs),
            (Index::Inclusive(lhs), Index::Exclusive(rhs))
            | (Index::Exclusive(lhs), Index::Exclusive(rhs)) => lhs.saturating_sub(rhs + 1),
        }
    }
}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        match *arr1.as_slice() {
            [_single_item] => return 0,
            [a, b] => {
                let check_single_swap = |x: i32| x > a || x < b;

                // [10, 1], [9, 2, 8, 3, 7, 4]
                let (&head, arr2) = arr2.split_first().expect("Input is not empty.");
                if check_single_swap(head) {
                    return 1;
                }

                let mut min = head;
                let mut max = head;
                for &y in arr2 {
                    if check_single_swap(y) {
                        return 1;
                    }

                    if y < min {
                        min = y;
                    }

                    if y > max {
                        max = y;
                    }
                }

                if min < max {
                    return 2;
                } else {
                    return -1;
                }
            }
            _ => (),
        }

        arr2.sort_unstable();
        arr2.dedup();

        // Build a "map" of indices of replacement for every number in `arr1`.
        //
        // For all `i` where `0 <= i < arr1.len()`,
        // we have `arr2[..memo[i]] < arr1[i] <= arr2[memo[i]..]`.
        //
        // So given `i` and `j` where `0 <= i < j < arr1.len()`,
        // we have `arr1[i] <= arr2[memo[i]..memo[j]] < arr1[j]`.
        // Or alternatively, `arr1[i] < arr2[k..memo[j]] < arr1[j]`,
        // where `k = memo[i]` if `arr1[i] != arr2[memo[i]]` else `k = memo[i] + 1`.
        // From this, we can know there are `memo[j] - k` numbers in `arr2` that
        // fit between `arr1[i]` and `arr1[j]`.
        let memo: Vec<Index> = arr1
            .iter()
            .map(|x| match arr2.binary_search(x) {
                Ok(i) => Index::Exclusive(i),
                Err(i) => Index::Inclusive(i),
            })
            .collect();

        let mut masks = vec![false; arr1.len()];

        'outer: for (i, window) in arr1.windows(2).enumerate() {
            let &[a, b] = window else { unreachable!() };

            if a < b {
                continue;
            }

            for n in 1..=min(arr1.len(), arr2.len()) {
                // `end - start = n`; `n` is the length of the slicing.
                // `arr1[start..end]` is the slice that we're looking to replace in order to fix
                // it's ordering. So it can only be as long as the shorter of `arr1` and `arr2`.

                // We want as small an `n` as possible, so we want to explore all the possible
                // slices that contain either `a` or `b`, or both.
                // We want all possible `start` and `end` given `n`,
                // where `0 <= start <= i + 1` (starting on `b` or before),
                // and `arr1.len() >= end >= i + 1` (ending on and including `a` or after).

                // Since `end = start + n` and `start <= i + 1`, `end <= i + 1 + n`.
                // We want `start >= 0`; we have `start = end - n` and `end >= i + 1`,
                // therefore we want `i + 1 - n >= 0`, therefore `i + 1 >= n`
                for end in max(n, i + 1)..=min(i + n + 1, arr1.len()) {
                    let start = end - n;

                    let min_index = if start == 0 {
                        Index::Inclusive(0)
                    } else {
                        memo[start - 1]
                    };

                    let max_index = if end == memo.len() {
                        Index::Inclusive(arr2.len())
                    } else {
                        memo[end]
                    };

                    if n <= max_index - min_index {
                        masks[start..end].set_all(true);
                        continue 'outer;
                    }
                }
            }

            return -1;
        }

        masks.into_iter().filter(|&x| x).count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4])
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1])
        );
        assert_eq!(
            -1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3])
        );
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![1, 3, 5, 4], vec![4, 5, 6, 7]) // [1, 3, 5, *7*]
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![1, 3, 2, 4, 7], vec![0, 4, 5, 6]) // [1, 3, *4*, *5*, 7]
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![2, 4, 3, 1, 7], vec![0, 4, 5, 6]) // [2, 4, *5*, *6*, 7]
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![2, 4, 3, 1, 7], vec![0, 1, 3, 5, 6]) // [2, 4, *5*, *6*, 7] | [*0*, *1*, 3, *5|6*, 7]
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![1, 2, 3, 10, 4], vec![0, 1, 2, 3, 4, 5]) // [1, 2, 3, *4*, *5*]
        );
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 2, 3, 4]) // [1, *2*, 3, 6, 7]
        );
        assert_eq!(
            2,
            Solution::make_array_increasing(vec![1, 5, 3, 4, 8, 6], vec![1, 2, 3, 4, 5]) // [1, *2*, 3, 4, *5*, 6]
        );
        assert_eq!(
            3,
            Solution::make_array_increasing(vec![1, 5, 3, 3, 4, 5], vec![0, 1, 2]) // [*0*, *1*, *2*, 3, 4, 5]
        );
    }
}
