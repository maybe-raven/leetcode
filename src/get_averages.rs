//! 2090. K Radius Subarray Averages
//! https://leetcode.com/problems/k-radius-subarray-averages

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let window_size = 2 * k + 1;
        let window_size_f = window_size as f64;
        let mut results = vec![-1; nums.len()];

        if nums.len() < window_size {
            return results;
        }

        let mut average =
            nums[..window_size].iter().map(|&x| x as f64).sum::<f64>() / window_size_f;
        results[k] = average.trunc() as i32;
        average -= nums[0] as f64 / window_size_f;

        for (window, out) in nums
            .windows(window_size)
            .zip(results.iter_mut().skip(k))
            .skip(1)
        {
            average += *window.last().unwrap() as f64 / window_size_f;
            *out = average.trunc() as i32;
            average -= *window.first().unwrap() as f64 / window_size_f;
        }

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3)
        );
        assert_eq!(vec![100000], Solution::get_averages(vec![100000], 0));
        assert_eq!(vec![-1], Solution::get_averages(vec![8], 10));
    }
}
