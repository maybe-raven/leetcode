//! 1732. Find the Highest Altitude
//! https://leetcode.com/problems/find-the-highest-altitude

use std::cmp::max;

impl Solution {
    pub fn largest_altitude(gains: Vec<i32>) -> i32 {
        let (&head, gains) = gains.split_first().expect("");

        let mut altitude = head;
        let mut max_altitude = max(0, altitude);

        for gain in gains {
            altitude += gain;
            max_altitude = max(max_altitude, altitude);
        }

        max_altitude
    }
}

pub struct Solution;
