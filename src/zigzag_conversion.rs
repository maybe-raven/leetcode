//! 6. Zigzag Conversion
//! https://leetcode.com/problems/zigzag-conversion

// Chunk every 2n - 2
// There will be s.len() / (2n - 2) (rounding up) chunks.

// First line is first element of every chunk,
// Second line is second and last element of every chunk,
// Third line is third and second last element of every chunk,
// Line i is chunk[i] and (chunk[chunk.len() - i] or None)

// The xth element of jth chunk:
// chunks[j][x] == s[(2n - 2) * j + x]

// Each chunk takes up 2n - 2 spaces horizontally.

// First and last line have 2n - 3 spaces between characters.
// On line i, there are 2i - 1 spaces between chunks,
// and 2n - 2i - 3 spaces between characters within a chunk.

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let max_i = num_rows - 1;
        let chunk_len = num_rows * 2 - 2;

        if chunk_len == 0 {
            return s;
        }

        let mut result = String::with_capacity(s.capacity());
        let s: Vec<char> = s.chars().collect();

        let n_chunks = f64::ceil((s.len() as f64) / (chunk_len as f64)) as usize;

        for i in 0..num_rows {
            for j in 0..n_chunks {
                if let Some(&ch) = s.get(chunk_len * j + i) {
                    result.push(ch);
                }

                if i == 0 || i == max_i {
                    continue;
                }

                if let Some(&ch) = s.get(chunk_len * (j + 1) - i) {
                    result.push(ch);
                }
            }
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
        assert_eq!("A".to_string(), Solution::convert("A".to_string(), 1));
        assert_eq!("AB".to_string(), Solution::convert("AB".to_string(), 1));
    }
}
