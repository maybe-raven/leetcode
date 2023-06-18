//! 1187. Make Array Strictly Increasing
//! https://leetcode.com/problems/make-array-strictly-increasing

use std::ops::RangeBounds;

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

trait CheckedDec: Sized {
    fn checked_dec(self) -> Option<Self>;
}

trait CheckedInc: Sized {
    fn checked_inc(self) -> Option<Self>;
}

macro_rules! impl_checked_dec {
    ($t:ty) => {
        impl CheckedDec for $t {
            fn checked_dec(self) -> Option<Self> {
                if self == Self::MIN {
                    None
                } else {
                    Some(self - 1)
                }
            }
        }
    };
}

macro_rules! impl_checked_inc {
    ($t:ty) => {
        impl CheckedInc for $t {
            fn checked_inc(self) -> Option<Self> {
                if self == Self::MAX {
                    None
                } else {
                    Some(self + 1)
                }
            }
        }
    };
}

impl_checked_dec! { usize }
impl_checked_inc! { usize }

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

impl<T: CheckedDec + CheckedInc + Copy, R: RangeBounds<T>> RangeGet<T> for R {
    fn get_exclusive_start(&self) -> Option<T> {
        match self.start_bound() {
            std::ops::Bound::Included(i) => i.checked_dec(),
            std::ops::Bound::Excluded(&i) => Some(i),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_exclusive_end(&self) -> Option<T> {
        match self.end_bound() {
            std::ops::Bound::Included(i) => i.checked_inc(),
            std::ops::Bound::Excluded(&i) => Some(i),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_start(&self) -> Option<T> {
        match self.start_bound() {
            std::ops::Bound::Included(&i) => Some(i),
            std::ops::Bound::Excluded(i) => i.checked_inc(),
            std::ops::Bound::Unbounded => None,
        }
    }

    fn get_end(&self) -> Option<T> {
        match self.end_bound() {
            std::ops::Bound::Included(&i) => Some(i),
            std::ops::Bound::Excluded(i) => i.checked_dec(),
            std::ops::Bound::Unbounded => None,
        }
    }
}

struct OrderedWindow<'a, T, R> {
    source: &'a [T],
    range: R,
}

impl<'a, T, R> OrderedWindow<'a, T, R> {
    fn new<I: Into<R>>(source: &'a [T], rangeable: I) -> Self {
        OrderedWindow {
            source,
            range: rangeable.into(),
        }
    }
}

impl<T: Ord, R: RangeBounds<usize>> OrderedWindow<'_, T, R> {
    fn len(&self) -> usize {
        let start = self.range.get_start().unwrap_or(0);
        let end = self.range.get_exclusive_end().unwrap_or(self.source.len());
        end - start
    }
}

impl<'a, T: Ord, R: RangeBounds<usize>> OrderedWindow<'a, T, R> {
    fn check_replacement(&self, replacement: &[T]) -> bool {
        // assert!(replacement.is_sorted());
        assert_eq!(self.len(), replacement.len());

        const ERRMSG: &str = "`replacement` must not be empty.";

        let left_is_ordered = if let Some(left_index) = self.range.get_exclusive_start() {
            &self.source[left_index] < replacement.first().expect(ERRMSG)
        } else {
            true
        };

        let right_is_ordered = if let Some(right_index) = self.range.get_exclusive_end() {
            if right_index < self.source.len() {
                &self.source[right_index] < replacement.last().expect(ERRMSG)
            } else {
                true
            }
        } else {
            true
        };

        unimplemented!()
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

        // [1, 3, 2, 4, 7], [0, 4, 5, 6]

        #[allow(unused)]
        for (i, window) in arr1.windows(2).enumerate() {
            let &[a, b] = window else { unreachable!() };

            if a < b {
                continue;
            }

            let mut n = 1;
            loop {
                for offset in 0..n {
                    let start = i - offset;
                    let range = start..start + n;
                }
            }

            unimplemented!()
        }

        unimplemented!()
    }
}

pub struct Solution;
