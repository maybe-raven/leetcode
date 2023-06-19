//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

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

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        if arr1.len() <= 1 {
            return 0;
        }

        let &head = arr1
            .first()
            .expect("`arr1` has at least 2 elements past early return.");

        let arr2 = {
            let mut arr2 = arr2;
            arr2.sort_unstable();
            arr2.dedup();
            arr2
        };

        let mut min_swaps_keeping_previous_original = Some(0);
        let mut next_replacements: Vec<(usize, usize)> = Vec::new();
        let mut previous_replacements: Vec<(usize, usize)> = Vec::new();
        if head != arr2[0] {
            previous_replacements.push((0, 1));
        }

        for window in arr1.windows(2) {
            let &[a, b] = window else { unreachable!() };

            let mut min_swaps_keeping_current_original = None;

            if let Some(previous_min) = min_swaps_keeping_previous_original {
                if a < b {
                    min_swaps_keeping_current_original = min_swaps_keeping_previous_original;
                }

                let i = match arr2.binary_search(&a) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };

                if arr2.get(i).is_some_and(|x| x != &b) {
                    next_replacements.push((i, previous_min + 1));
                }
            }

            for (mut replacement_index, mut replacement_count) in previous_replacements.drain(..) {
                let a = arr2[replacement_index];

                if a < b && min_swaps_keeping_current_original.is_none_or(|x| replacement_count < x)
                {
                    min_swaps_keeping_current_original = Some(replacement_count);
                }

                replacement_index += 1;
                replacement_count += 1;

                if arr2.get(replacement_index).is_some_and(|x| x != &b) {
                    next_replacements.push((replacement_index, replacement_count));
                }
            }

            min_swaps_keeping_previous_original = min_swaps_keeping_current_original;
            (previous_replacements, next_replacements) = (next_replacements, previous_replacements);
        }

        previous_replacements
            .into_iter()
            .map(|x| x.1 as i32)
            .chain(min_swaps_keeping_previous_original.map(|x| x as i32))
            .min()
            .unwrap_or(-1)
    }
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
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]) // [1, *3*, *4*, 6, 7]
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
