//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

use std::{
    cmp::{max, min},
    ops::Sub,
};

trait IsNoneOr<T>: Copy {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T: Copy> IsNoneOr<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Some(x) => f(x),
            None => true,
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
        if arr1.len() <= 1 {
            return 0;
        }

        arr2.sort_unstable();
        arr2.dedup();

        // Build a "map" of indices of replacement for every number in `arr1`.
        //
        // For all `i` where `0 <= i < arr1.len()`,
        // we have `arr2[..indexer[i]] < arr1[i] <= arr2[indexer[i]..]`.
        //
        // So given `i` and `j` where `0 <= i < j < arr1.len()`,
        // we have `arr1[i] <= arr2[indexer[i]..indexer[j]] < arr1[j]`.
        // Or alternatively, `arr1[i] < arr2[k..indexer[j]] < arr1[j]`,
        // where `k = indexer[i]` if `arr1[i] != arr2[indexer[i]]` else `k = indexer[i] + 1`.
        // From this, we can know there are `indexer[j] - k` numbers in `arr2` that
        // fit between `arr1[i]` and `arr1[j]`.
        let indexer: Vec<Index> = arr1
            .iter()
            .map(|x| match arr2.binary_search(x) {
                Ok(i) => Index::Exclusive(i),
                Err(i) => Index::Inclusive(i),
            })
            .collect();

        solutionate(&arr1, &indexer, arr2.len(), 0).map_or(-1, |x| x as i32)
    }
}

/// Find the mininum number of swicheroos needed to make `arr1[narnia..]` strictly increasing,
/// without touching `arr1[..narnia]`, as that part is narnia business.
fn solutionate(
    arr1: &[i32],
    indexonomicon: &[Index],
    arr2_len: usize,
    narnia: usize,
) -> Option<usize> {
    for (i, window) in arr1.windows(2).enumerate().skip(narnia) {
        let &[a, b] = window else { unreachable!() };

        if a < b {
            continue;
        }

        let mut mintendo_switch = None;

        for switcheroo_size in 1..=min(arr1.len() - narnia, arr2_len) {
            // `end - start = n`; `n` is the length of the slicing.
            // `arr1[start..end]` is the slice that we're looking to replace in order to fix
            // it's ordering. So it can only be as long as the shorter of `arr1` and `arr2`.

            // We want as small an `n` as possible, so we want to explore all the possible
            // slices that contain either `a` or `b`, or both.
            // We want all possible `start` and `end` given `n`,
            // where `narnia <= start <= i + 1` (starting on `b` or before),
            // and `arr1.len() >= end >= i + 1` (ending on and including `a` or after).

            // Since `end = start + n` and `start <= i + 1`, `end <= i + 1 + n`.
            // We want `start >= narnia`; we have `start = end - n` and `end >= i + 1`,
            // therefore we want `i + 1 - n >= narnia`, therefore `i + 1 >= n + narnia`
            for end in
                max(switcheroo_size + narnia, i + 1)..=min(i + switcheroo_size + 1, arr1.len())
            {
                let start = end - switcheroo_size;

                let min_index = if start == 0 {
                    Index::Inclusive(0)
                } else {
                    indexonomicon[start - 1]
                };

                let max_index = if end == indexonomicon.len() {
                    Index::Inclusive(arr2_len)
                } else {
                    indexonomicon[end]
                };

                if switcheroo_size <= max_index - min_index {
                    let Some(switcheroo_size) = solutionate(arr1, indexonomicon, arr2_len, end)
                        .map(|n| n + switcheroo_size)
                    else {
                        continue;
                    };

                    if mintendo_switch.is_none_or(|x| switcheroo_size < x) {
                        mintendo_switch = Some(switcheroo_size);
                    }
                }
            }
        }

        return mintendo_switch;
    }

    Some(0)
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(0, Solution::make_array_increasing(vec![5], vec![2]));
        assert_eq!(-1, Solution::make_array_increasing(vec![5, 1], vec![2]));
        assert_eq!(
            0,
            Solution::make_array_increasing(vec![1, 2], vec![1, 3, 2, 4])
        );
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![2, 1], vec![1, 3, 2, 4])
        );
        assert_eq!(
            1,
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]) // [1, *2*, 3, 6, 7]
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
            -1,
            Solution::make_array_increasing(vec![1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 2, 3, 4, 5]) // [1, 2, 3, *4*, *5*]
        );
        assert_eq!(
            5,
            Solution::make_array_increasing(vec![1, 1, 1, 1, 1, 1], vec![0, 1, 2, 3, 4, 5]) // [1, 2, 3, *4*, *5*]
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
        assert_eq!(
            -1,
            Solution::make_array_increasing(vec![1, 5, 3, 3, 4, 5], vec![1, 2, 5])
        );
        assert_eq!(
            3,
            Solution::make_array_increasing(vec![7, 8, 9, 4, 5], vec![1, 2, 3])
        );
    }
}
