//! 1732. Find the Highest Altitude
//! https://leetcode.com/problems/find-the-highest-altitude

use std::cmp::max;

trait MaxAssign<Rhs = Self> {
    fn max_assign(&mut self, other: Rhs);
}

impl MaxAssign for i32 {
    fn max_assign(&mut self, other: Self) {
        *self = (*self).max(other);
    }
}

impl Solution {
    pub fn largest_altitude(gains: Vec<i32>) -> i32 {
        let (&head, gains) = gains.split_first().expect("");

        let mut altitude = head;
        let mut max_altitude = max(0, altitude);

        for gain in gains {
            altitude += gain;
            max_altitude.max_assign(altitude);
        }

        max_altitude
    }
}

pub struct Solution;
