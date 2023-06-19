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

#[derive(Debug, Clone, Copy)]
enum Number {
    Original(i32),
    Replaced(usize),
}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        if arr1.len() <= 1 {
            return 0;
        }

        let (&head, tail) = arr1
            .split_first()
            .expect("`arr1` has at least 2 elements past early return.");

        let arr2 = {
            let mut arr2 = arr2;
            arr2.sort_unstable();
            arr2.dedup();
            arr2
        };

        let mut current: Vec<(Number, usize)> = Vec::new();
        let mut next: Vec<(Number, usize)> = Vec::new();
        current.push((Number::Original(head), 0));
        if head != arr2[0] {
            current.push((Number::Replaced(0), 1));
        }

        for &x in tail {
            let mut min_swaps_for_original = None;

            for (num, swaps) in current.drain(..) {
                let mut check_and_update_min = |previous: i32| {
                    if previous < x && min_swaps_for_original.is_none_or(|c| swaps < c) {
                        min_swaps_for_original = Some(swaps);
                    }
                };

                let check_replacement_at_index = |i: usize| {
                    if i < arr2.len() && arr2[i] != x {
                        Some(i)
                    } else {
                        None
                    }
                };

                if let Some(i) = match num {
                    Number::Original(a) => {
                        check_and_update_min(a);

                        let i = match arr2.binary_search(&a) {
                            Ok(i) => i + 1,
                            Err(i) => i,
                        };

                        check_replacement_at_index(i)
                    }
                    Number::Replaced(i) => {
                        check_and_update_min(arr2[i]);
                        check_replacement_at_index(i + 1)
                    }
                } {
                    next.push((Number::Replaced(i), swaps + 1));
                }
            }

            if let Some(swaps) = min_swaps_for_original {
                next.push((Number::Original(x), swaps));
            }

            (current, next) = (next, current);
        }

        current.into_iter().map(|x| x.1 as i32).min().unwrap_or(-1)
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
