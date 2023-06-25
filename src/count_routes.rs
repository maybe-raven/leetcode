//! 1575. Count All Possible Routes
//! https://leetcode.com/problems/count-all-possible-routes

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        unimplemented!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(4, Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5));
        assert_eq!(5, Solution::count_routes(vec![4, 3, 1], 1, 0, 6));
    }
}
