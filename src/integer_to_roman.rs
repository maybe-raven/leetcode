//! https://leetcode.com/problems/integer-to-roman/
//!
//! I => 1
//! V => 5
//! X => 10
//! L => 50
//! C => 100
//! D => 500
//! M => 1000
//!
//! I can be placed before V (5) and X (10) to make 4 and 9.
//! X can be placed before L (50) and C (100) to make 40 and 90.
//! C can be placed before D (500) and M (1000) to make 400 and 900.

#[derive(Debug, PartialEq, Eq)]
struct RomanNumeral {
    thousands: u8,
    hundreds: u8,
    tens: u8,
    ones: u8,
}

impl RomanNumeral {
    fn thousands_str(&self) -> Option<&str> {
        match self.thousands {
            0 => Some(""),
            1 => Some("M"),
            2 => Some("MM"),
            3 => Some("MMM"),
            _ => None,
        }
    }

    fn hundreds_str(&self) -> Option<&str> {
        match self.hundreds {
            0 => Some(""),
            1 => Some("C"),
            2 => Some("CC"),
            3 => Some("CCC"),
            4 => Some("CD"),
            5 => Some("D"),
            6 => Some("DC"),
            7 => Some("DCC"),
            8 => Some("DCCC"),
            9 => Some("CM"),
            _ => None,
        }
    }
    fn tens_str(&self) -> Option<&str> {
        match self.tens {
            0 => Some(""),
            1 => Some("X"),
            2 => Some("XX"),
            3 => Some("XXX"),
            4 => Some("XL"),
            5 => Some("L"),
            6 => Some("LX"),
            7 => Some("LXX"),
            8 => Some("LXXX"),
            9 => Some("XC"),
            _ => None,
        }
    }
    fn ones_str(&self) -> Option<&str> {
        match self.ones {
            0 => Some(""),
            1 => Some("I"),
            2 => Some("II"),
            3 => Some("III"),
            4 => Some("IV"),
            5 => Some("V"),
            6 => Some("VI"),
            7 => Some("VII"),
            8 => Some("VIII"),
            9 => Some("IX"),
            _ => None,
        }
    }
}

impl From<RomanNumeral> for Option<String> {
    fn from(value: RomanNumeral) -> Self {
        let mut output = String::with_capacity(56);
        output.push_str(value.thousands_str()?);
        output.push_str(value.hundreds_str()?);
        output.push_str(value.tens_str()?);
        output.push_str(value.ones_str()?);
        Some(output)
    }
}

impl From<i32> for RomanNumeral {
    fn from(mut value: i32) -> Self {
        let thousands = value / 1000;
        value %= 1000;
        let hundres = value / 100;
        value %= 100;
        let tens = value / 10;
        value %= 10;
        let ones = value;
        Self {
            thousands: thousands as u8,
            hundreds: hundres as u8,
            tens: tens as u8,
            ones: ones as u8,
        }
    }
}

impl Solution {
    /// 1 <= num <= 3999
    pub fn int_to_roman(num: i32) -> String {
        Option::<String>::from(RomanNumeral::from(num)).unwrap()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numeral_ones() {
        let three = RomanNumeral {
            thousands: 0,
            hundreds: 0,
            tens: 0,
            ones: 3,
        };
        let fifty_eight = RomanNumeral {
            thousands: 0,
            hundreds: 0,
            tens: 5,
            ones: 8,
        };
        let nineteen_ninety_four = RomanNumeral {
            thousands: 1,
            hundreds: 9,
            tens: 9,
            ones: 4,
        };
        let thirty_nine_ninety_nine = RomanNumeral {
            thousands: 3,
            hundreds: 9,
            tens: 9,
            ones: 9,
        };
        assert_eq!(three.ones_str().unwrap(), "III");
        assert_eq!(fifty_eight.ones_str().unwrap(), "VIII");
        assert_eq!(nineteen_ninety_four.ones_str().unwrap(), "IV");
        assert_eq!(thirty_nine_ninety_nine.ones_str().unwrap(), "IX");
    }

    #[test]
    fn test_roman_numeral_from_i32() {
        let three = RomanNumeral {
            thousands: 0,
            hundreds: 0,
            tens: 0,
            ones: 3,
        };
        let fifty_eight = RomanNumeral {
            thousands: 0,
            hundreds: 0,
            tens: 5,
            ones: 8,
        };
        let nineteen_ninety_four = RomanNumeral {
            thousands: 1,
            hundreds: 9,
            tens: 9,
            ones: 4,
        };
        let thirty_nine_ninety_nine = RomanNumeral {
            thousands: 3,
            hundreds: 9,
            tens: 9,
            ones: 9,
        };
        assert_eq!(RomanNumeral::from(3), three);
        assert_eq!(RomanNumeral::from(58), fifty_eight);
        assert_eq!(RomanNumeral::from(1994), nineteen_ninety_four);
        assert_eq!(RomanNumeral::from(3999), thirty_nine_ninety_nine);
    }

    #[test]
    fn test_solution() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
        assert_eq!(Solution::int_to_roman(3888), "MMMDCCCLXXXVIII");
    }
}
