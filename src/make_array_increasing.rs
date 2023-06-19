//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
enum Number {
    Original(i32),
    Replaced(usize),
}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        if arr1.len() <= 1 {
            return 0;
        }

        arr2.sort_unstable();
        arr2.dedup();

        let memo: Vec<RefCell<Vec<(Number, usize)>>> = vec![RefCell::new(Vec::new()); arr1.len()];

        memo[0].borrow_mut().push((Number::Original(arr1[0]), 0));
        if arr1[0] != arr2[0] {
            memo[0].borrow_mut().push((Number::Replaced(0), 1));
        }

        for (memo_window, x) in memo.windows(2).zip(arr1) {
            let [previous_states, current_states] = memo_window else { unreachable!() };

            for &state in previous_states.borrow().iter() {
                match state {
                    (Number::Original(y), swap_count) => {
                        if y < x {
                            current_states
                                .borrow_mut()
                                .push((Number::Original(x), swap_count));
                        }

                        if let Some(i) = match arr2.binary_search(&y) {
                            Ok(i) | Err(i) if i < arr2.len() - 1 => Some(i + 1),
                            Ok(_) | Err(_) => None,
                        } {
                            current_states
                                .borrow_mut()
                                .push((Number::Replaced(i), swap_count + 1))
                        }
                    }
                    (Number::Replaced(mut i), swap_count) => {
                        if arr2[i] < x {
                            current_states
                                .borrow_mut()
                                .push((Number::Original(x), swap_count));
                        }

                        i += 1;
                        if i < arr2.len() {
                            current_states
                                .borrow_mut()
                                .push((Number::Replaced(i), swap_count + 1));
                        }
                    }
                }

                unimplemented!()
            }

            todo!()
        }

        unimplemented!()
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
