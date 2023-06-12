//! 228. Summary Ranges
//! https://leetcode.com/problems/summary-ranges/

struct Range {
    start: i32,
    end: i32,
}

impl ToString for Range {
    fn to_string(&self) -> String {
        if self.end == self.start {
            self.end.to_string()
        } else {
            let a = self.start.to_string();
            let b = self.end.to_string();
            let mut string = String::with_capacity(a.len() + b.len() + 2);

            string.push_str(a.as_str());
            string.push_str("->");
            string.push_str(b.as_str());

            string
        }
    }
}

#[derive(Debug)]
struct MapContiguousRanges<I> {
    start: i32,
    last: i32,
    iter: Option<I>,
}

impl<I> Default for MapContiguousRanges<I> {
    fn default() -> Self {
        Self {
            start: 0,
            last: 0,
            iter: None,
        }
    }
}

impl<I> MapContiguousRanges<I>
where
    I: Iterator<Item = i32>,
{
    fn new(mut iter: I) -> Self {
        if let Some(head) = iter.next() {
            Self {
                start: head,
                last: head,
                iter: Some(iter),
            }
        } else {
            Self::default()
        }
    }
}

impl<I> Iterator for MapContiguousRanges<I>
where
    I: Iterator<Item = i32>,
{
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(iter) = &mut self.iter else {return None; };

        while let Some(x) = iter.next() {
            if self.last == x - 1 {
                self.last = x;
                continue;
            }

            let range = Range {
                start: self.start,
                end: self.last,
            };
            self.start = x;
            self.last = x;
            return Some(range);
        }

        self.iter = None;
        Some(Range {
            start: self.start,
            end: self.last,
        })
    }
}

trait ToMapContiguousRanges: Iterator<Item = i32> + Sized {
    fn map_contiguous_ranges(self) -> MapContiguousRanges<Self> {
        MapContiguousRanges::new(self)
    }
}

impl<I> ToMapContiguousRanges for I where I: Iterator<Item = i32> + Sized {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        nums.into_iter()
            .map_contiguous_ranges()
            .map(|r| r.to_string())
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(Vec::<String>::new(), Solution::summary_ranges(vec![]));
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
