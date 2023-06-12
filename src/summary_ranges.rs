//! 228. Summary Ranges
//! https://leetcode.com/problems/summary-ranges/

fn add_range(results: &mut Vec<String>, start: i32, end: i32) {
    if end == start {
        results.push(end.to_string());
    } else {
        let a = start.to_string();
        let b = end.to_string();
        let mut range = String::with_capacity(a.len() + b.len() + 2);

        range.push_str(a.as_str());
        range.push_str("->");
        range.push_str(b.as_str());

        results.push(range)
    }
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut results = Vec::new();
        let (&(mut start), nums) = match nums.split_first() {
            Some(split_result) => split_result,
            None => return results,
        };
        let mut last = start;

        for &x in nums {
            if last != x - 1 {
                add_range(&mut results, start, last);
                start = x;
            }
            last = x;
        }

        add_range(&mut results, start, last);

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            vec!["0->2", "4->5", "7"],
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
        assert_eq!(
            vec!["0", "2->4", "6", "8->9"],
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
        assert_eq!(
            vec![
                "-10->-8",
                "-5",
                "-2->1",
                "3->4",
                "6",
                "8",
                "10->11",
                "100",
                "1000->1001",
                "2000",
                "20000"
            ],
            Solution::summary_ranges(vec![
                -10, -9, -8, -5, -2, -1, 0, 1, 3, 4, 6, 8, 10, 11, 100, 1000, 1001, 2000, 20000
            ])
        );
    }
}
