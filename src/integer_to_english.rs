//! https://leetcode.com/problems/integer-to-english-words

fn special_cases(value: i32) -> Option<&'static str> {
    match value {
        10 => Some("Ten"),
        11 => Some("Eleven"),
        12 => Some("Twelve"),
        13 => Some("Thirteen"),
        14 => Some("Fourteen"),
        15 => Some("Fifteen"),
        16 => Some("Sixteen"),
        17 => Some("Seventeen"),
        18 => Some("Eighteen"),
        19 => Some("Nineteen"),
        _ => None,
    }
}

fn parse_tens(digit: i32) -> Option<&'static str> {
    match digit {
        2 => Some("Twenty"),
        3 => Some("Thirty"),
        4 => Some("Forty"),
        5 => Some("Fifty"),
        6 => Some("Sixty"),
        7 => Some("Seventy"),
        8 => Some("Eighty"),
        9 => Some("Ninety"),
        _ => None,
    }
}

fn parse_digit(digit: i32) -> Option<&'static str> {
    match digit {
        1 => Some("One"),
        2 => Some("Two"),
        3 => Some("Three"),
        4 => Some("Four"),
        5 => Some("Five"),
        6 => Some("Six"),
        7 => Some("Seven"),
        8 => Some("Eight"),
        9 => Some("Nine"),
        _ => None,
    }
}

fn parse_3_digit_num(mut num: i32, out: &mut Vec<&'static str>) {
    num %= 1000;

    if num >= 100 {
        out.push(parse_digit(num / 100).unwrap());
        out.push("Hundred");
        num %= 100;
    }

    if let Some(s) = special_cases(num) {
        out.push(s);
        return;
    }

    if num >= 20 {
        out.push(parse_tens(num / 10).unwrap());
    }

    if let Some(ones) = parse_digit(num % 10) {
        out.push(ones);
    }
}

const ONE_BILLION: i32 = 1000000000;
const ONE_MILLION: i32 = 1000000;
const ONE_THOUSAND: i32 = 1000;

impl Solution {
    // 0 <=> num <=> 231 - 1
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let mut out = Vec::new();

        if num >= ONE_BILLION {
            parse_3_digit_num(num / ONE_BILLION, &mut out);
            out.push("Billion");
            num %= ONE_BILLION;
        }

        if num >= ONE_MILLION {
            parse_3_digit_num(num / ONE_MILLION, &mut out);
            out.push("Million");
            num %= ONE_MILLION;
        }

        if num >= ONE_THOUSAND {
            parse_3_digit_num(num / ONE_THOUSAND, &mut out);
            out.push("Thousand");
            num %= ONE_THOUSAND;
        }

        parse_3_digit_num(num, &mut out);

        out.join(" ")
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::number_to_words(100), "One Hundred");
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
        assert_eq!(Solution::number_to_words(103), "One Hundred Three");
        assert_eq!(Solution::number_to_words(1000), "One Thousand");
        assert_eq!(Solution::number_to_words(1000000), "One Million");
        assert_eq!(
            Solution::number_to_words(1234),
            "One Thousand Two Hundred Thirty Four"
        );
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }
}
