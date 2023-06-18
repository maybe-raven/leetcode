//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

use std::{
    cmp::min,
    marker::PhantomData,
    ops::{Range, RangeBounds},
};

// trait CappedRange {
//     fn cap(&self, source: &[T]) -> impl RangeBounds;
// }
//
// impl CappedRange for Range<i32> {
//     fn cap(&self, source: &[T]) -> impl RangeBounds {
//         match (self.start.cmp(0), self.end.cmp(source.len())) {
//
//         }
//         // if self.start < 0 && self.end > source.len() {
//         //     ..
//         // } else if self.start < 0 {
//         //     ..self.end
//         // } else if self.end
//         // match self.start {
//         //
//         // }
//         // match self.start_bound() {
//         //
//         // }
//         todo!()
//     }
// }

trait Dec: Sized {
    fn dec(self) -> Self;
}

trait Inc: Sized {
    fn inc(self) -> Self;
}

macro_rules! impl_dec {
    ($t:ty) => {
        impl Dec for $t {
            fn dec(self) -> Self {
                self - 1
            }
        }
    };
}

macro_rules! impl_inc {
    ($t:ty) => {
        impl Inc for $t {
            fn inc(self) -> Self {
                self + 1
            }
        }
    };
}

impl_dec! { i32 }
impl_inc! { i32 }

trait RangeOrd<T> {
    fn range_lt(&self, other: &T) -> bool;
    fn range_gt(&self, other: &T) -> bool;
}

impl<T: PartialOrd, R: RangeBounds<T>> RangeOrd<T> for R {
    fn range_lt(&self, other: &T) -> bool {
        match self.end_bound() {
            std::ops::Bound::Included(i) => i < other,
            std::ops::Bound::Excluded(i) => i <= other,
            std::ops::Bound::Unbounded => false,
        }
    }

    fn range_gt(&self, other: &T) -> bool {
        match self.start_bound() {
            std::ops::Bound::Included(i) => i > other,
            std::ops::Bound::Excluded(i) => i >= other,
            std::ops::Bound::Unbounded => false,
        }
    }
}

trait RangeGet<T> {
    /// Gets the value that is "one less than" the start.
    fn get_exclusive_start(&self) -> Option<T>;
    /// Gets the value that is inclusively the start.
    fn get_start(&self) -> Option<T>;
    /// Gets the value that is "one more than" the end.
    fn get_exclusive_end(&self) -> Option<T>;
    /// Gets the value that is inclusively the end.
    fn get_end(&self) -> Option<T>;
}

impl<T: Dec + Inc + Copy, R: RangeBounds<T>> RangeGet<T> for R {
    fn get_exclusive_start(&self) -> Option<T> {
        match self.start_bound() {
            std::ops::Bound::Included(i) => Some(i.dec()),
            std::ops::Bound::Excluded(&i) => Some(i),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_exclusive_end(&self) -> Option<T> {
        match self.end_bound() {
            std::ops::Bound::Included(i) => Some(i.inc()),
            std::ops::Bound::Excluded(&i) => Some(i),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_start(&self) -> Option<T> {
        match self.start_bound() {
            std::ops::Bound::Included(&i) => Some(i),
            std::ops::Bound::Excluded(i) => Some(i.inc()),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_end(&self) -> Option<T> {
        match self.end_bound() {
            std::ops::Bound::Included(&i) => Some(i),
            std::ops::Bound::Excluded(i) => Some(i.dec()),
            std::ops::Bound::Unbounded => None,
        }
    }
}

#[allow(dead_code)]
struct OrderedWindow<'a, T, I, R> {
    source: &'a [T],
    range: R,
    i: PhantomData<I>,
}

#[allow(dead_code)]
impl<'a, T, I, R> OrderedWindow<'a, T, I, R> {
    fn new(source: &'a [T], range: R) -> Self {
        OrderedWindow {
            source,
            range,
            i: PhantomData,
        }
    }
}

#[allow(dead_code)]
impl<T, I: TryInto<usize> + Dec + Inc + Copy, R: RangeBounds<I>> OrderedWindow<'_, T, I, R> {
    fn len(&self) -> usize {
        let start = self
            .range
            .get_start()
            .and_then(|x| x.try_into().ok())
            .unwrap_or(0);
        let end = self
            .range
            .get_exclusive_end()
            .and_then(|x| x.try_into().ok())
            .unwrap_or(self.source.len());
        end - start
    }
}

#[allow(dead_code)]
impl<T: Ord, I: TryInto<usize> + Dec + Inc + Copy, R: RangeBounds<I>> OrderedWindow<'_, T, I, R> {
    fn check_replacement(&self, replacement: &[T]) -> bool {
        // assert!(replacement.is_sorted());
        assert_eq!(self.len(), replacement.len());

        const ERRMSG: &str = "`replacement` must not be empty.";

        let left_is_ordered = if let Some(left_index) = self
            .range
            .get_exclusive_start()
            .and_then(|x| x.try_into().ok())
        {
            &self.source[left_index] < replacement.first().expect(ERRMSG)
        } else {
            true
        };

        let right_is_ordered = if let Some(right_index) = self
            .range
            .get_exclusive_end()
            .and_then(|x| x.try_into().ok())
        {
            if right_index < self.source.len() {
                &self.source[right_index] < replacement.last().expect(ERRMSG)
            } else {
                true
            }
        } else {
            true
        };

        left_is_ordered && right_is_ordered
    }
}

trait CheckWindow {
    fn check_window(&self, start: usize, end: usize, replacement: &Self) -> bool;
}

impl<T: Ord> CheckWindow for [T] {
    fn check_window(&self, start: usize, end: usize, replacement: &Self) -> bool {
        // assert!(replacement.is_sorted());
        assert_eq!(end - start, replacement.len());

        const ERRMSG: &str = "`replacement` must not be empty.";

        let left_is_ordered = if start == 0 {
            true
        } else {
            &self[start - 1] < replacement.first().expect(ERRMSG)
        };

        let right_is_ordered = if end < self.len() {
            &self[end] > replacement.last().expect(ERRMSG)
        } else {
            true
        };

        left_is_ordered && right_is_ordered
    }
}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        match arr1.as_slice() {
            &[_single_item] => return 0,
            &[a, b] => {
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

        let memo: Vec<Option<usize>> = arr1
            .iter()
            .map(|x| {
                let i = arr2.partition_point(|y| y < x);
                if i == 0 {
                    None
                } else {
                    Some(i - 1)
                }
            })
            .collect();

        // [1, 3, 2, 4, 7], [0, 4, 5, 6]
        // [2, 4, 3, 1, 7], [0, 4, 5, 6]

        #[allow(unused)]
        for (i, window) in arr1.windows(2).enumerate() {
            let &[a, b] = window else { unreachable!() };

            if a < b {
                continue;
            }

            // i = 1
            // n = 1;
            // end starts at i + 1, goes up to i + n + 1;
            // start = end - n;
            // start in i + 1 - n .. i + 1
            // want [3] and [2]
            // 1..2 and 2..3
            // n = 2;
            // want [1, 3], [3, 2], and [2, 4]
            // 0..2, 1..3, and 2..4
            // n = 3;
            // -1..2, 0..3, 1..4, 2..5
            // -1..2 We want to ignore this one, because it's the same as 0..2.
            let mut n = 1;
            loop {
                for end in i + 1..min(i + n + 1, arr1.len()) {
                    if end < n {
                        continue;
                    }

                    let start = end - n;

                    for replacement in arr2.windows(n) {
                        if arr1.check_window(start, end, replacement) {
                            break;
                        }
                    }

                    unimplemented!()
                }
                // for offset in -n..1 {
                //     let start = i as i32 - offset;
                //     let range = start..start + n;
                // }
            }

            unimplemented!()
        }

        unimplemented!()
    }
}

pub struct Solution;
