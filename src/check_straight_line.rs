//! 1232. Check If It Is a Straight Line
//! https://leetcode.com/problems/check-if-it-is-a-straight-line/

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    slope: f64,
    intercept: f64,
}

impl Coordinate {
    fn new(input: &[i32]) -> Self {
        Self {
            x: *input.first().expect("Input must be in the form of [x, y]") as f64,
            y: *input.last().expect("Input must be in the form of [x, y]") as f64,
        }
    }
}

impl Line {
    fn new(first: Coordinate, second: Coordinate) -> Self {
        let slope = (first.y - second.y) / (first.x - second.x);
        let intercept = if slope.is_infinite() {
            first.x
        } else {
            first.y - slope * first.x
        };

        Self { slope, intercept }
    }

    fn contains(self, coordinate: Coordinate) -> bool {
        if self.slope.is_infinite() {
            coordinate.x == self.intercept
        } else {
            coordinate.y == self.slope * coordinate.x + self.intercept
        }
    }
}

impl Solution {
    /// # Constraints
    ///
    /// - `2 <= coordinates.len() <= 1000`
    /// - `coordinates[i].len() == 2`
    /// - `-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4`
    /// - `coordinates` contains no duplicate point.
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let line = Line::new(
            Coordinate::new(&coordinates[0]),
            Coordinate::new(&coordinates[1]),
        );

        coordinates[2..]
            .iter()
            .all(|coordinate| line.contains(Coordinate::new(coordinate)))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_straight_line() {
        assert!(Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]));
        assert!(!Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]));
        assert!(Solution::check_straight_line(vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, -1]
        ]));
    }
}
