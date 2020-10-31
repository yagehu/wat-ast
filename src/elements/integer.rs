use std::{fmt, result, str::FromStr};

use super::Sign;
use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Decimal {
    pub sign: Sign,
    pub num: String,
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(Error::EmptyExpr);
        }

        let sign: Sign;
        let chars: Vec<char> = s.chars().into_iter().collect();
        let mut num: Vec<char> = Vec::new();

        match chars[0] {
            '+' => {
                sign = Sign::Positive;
            }
            '-' => {
                sign = Sign::Negative;
            }
            '_' => {
                return Err(Error::InvalidChar {
                    idx: 0,
                    string: s.to_string(),
                });
            }
            _ => sign = Sign::Empty,
        }

        for (i, &c) in chars.iter().enumerate() {
            if i == 0 && (c == '+' || c == '-') {
                continue;
            }

            if i == chars.len() - 1 && c == '_' {
                return Err(Error::InvalidChar {
                    idx: i,
                    string: s.to_string(),
                });
            }

            if c == '0'
                || c == '1'
                || c == '2'
                || c == '3'
                || c == '4'
                || c == '5'
                || c == '6'
                || c == '7'
                || c == '8'
                || c == '9'
                || c == '_'
            {
                num.push(c);
            }
        }

        if num.len() == 0 {
            return Err(Error::MalformedPattern(s.to_string()));
        }

        Ok(Self {
            sign,
            num: num.iter().collect(),
        })
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, self.num,)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hexadecimal {
    sign: Sign,
    hexnum: String,
}

impl FromStr for Hexadecimal {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(Error::EmptyExpr);
        }

        let start: usize;
        let sign: Sign;
        let chars: Vec<char> = s.chars().into_iter().collect();

        match chars[0] {
            '+' => {
                sign = Sign::Positive;
                start = 1;
            }
            '-' => {
                sign = Sign::Negative;
                start = 1;
            }
            '_' => {
                return Err(Error::InvalidChar {
                    idx: 0,
                    string: s.to_string(),
                });
            }
            _ => {
                sign = Sign::Empty;
                start = 0;
            }
        }

        if chars.len() < start + 2 {
            return Err(Error::MalformedPattern(s.to_string()));
        }

        if chars[start] != '0' {
            return Err(Error::InvalidChar {
                idx: start,
                string: s.to_string(),
            });
        }

        if chars[start + 1] != 'x' {
            return Err(Error::InvalidChar {
                idx: start + 1,
                string: s.to_string(),
            });
        }

        let mut hexnum: Vec<char> = vec![];

        for (i, &c) in chars[start + 2..].iter().enumerate() {
            if i == 0 && (c == '+' || c == '-') {
                continue;
            }

            if i == chars.len() - 1 && c == '_' {
                return Err(Error::InvalidChar {
                    idx: i,
                    string: s.to_string(),
                });
            }

            if c == '0'
                || c == '1'
                || c == '2'
                || c == '3'
                || c == '4'
                || c == '5'
                || c == '6'
                || c == '7'
                || c == '8'
                || c == '9'
                || c == 'A'
                || c == 'a'
                || c == 'B'
                || c == 'b'
                || c == 'C'
                || c == 'c'
                || c == 'D'
                || c == 'd'
                || c == 'E'
                || c == 'e'
                || c == 'F'
                || c == 'f'
                || c == '_'
            {
                hexnum.push(c);
            }
        }

        if hexnum.len() == 0 {
            return Err(Error::MalformedPattern(s.to_string()));
        }

        Ok(Self {
            sign,
            hexnum: hexnum.iter().collect(),
        })
    }
}

impl fmt::Display for Hexadecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}0x{}", self.sign, self.hexnum)
    }
}

/// https://webassembly.github.io/spec/core/text/values.html#integers
#[derive(Clone, Debug, PartialEq)]
pub enum Integer {
    Decimal(Decimal),
    Hexadecimal(Hexadecimal),
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal(d) => write!(f, "{}", d),
            Self::Hexadecimal(h) => write!(f, "{}", h),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn decimal_from_str_empty() {
        if let Error::EmptyExpr = Decimal::from_str("").unwrap_err() {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn decimal_from_str_underscore() {
        if let Error::InvalidChar { idx, string } =
            Decimal::from_str("_").unwrap_err()
        {
            assert_eq!(idx, 0);
            assert_eq!(string, "_");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn decimal_from_str_number_without_underscore() {
        assert_eq!(
            Decimal::from_str("123").unwrap(),
            Decimal {
                sign: Sign::Empty,
                num: "123".to_string(),
            }
        )
    }

    #[test]
    fn decimal_from_str_positive_only() {
        if let Error::MalformedPattern(s) = Decimal::from_str("+").unwrap_err()
        {
            assert_eq!(s, "+");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn decimal_from_str_positive_number() {
        assert_eq!(
            Decimal::from_str("+123").unwrap(),
            Decimal {
                sign: Sign::Positive,
                num: "123".to_string(),
            },
        )
    }

    #[test]
    fn decimal_from_str_long_number() {
        assert_eq!(
            Decimal::from_str("123_456_789_123_456_789").unwrap(),
            Decimal {
                sign: Sign::Empty,
                num: "123_456_789_123_456_789".to_string(),
            },
        )
    }

    #[test]
    fn decimal_from_str_number_ends_with_underscore() {
        if let Error::InvalidChar { idx, string } =
            Decimal::from_str("123_").unwrap_err()
        {
            assert_eq!(idx, 3);
            assert_eq!(string, "123_");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hexadecimal_from_str_empty() {
        if let Error::EmptyExpr = Hexadecimal::from_str("").unwrap_err() {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hexadecimal_from_str_underscore() {
        if let Error::InvalidChar { idx, string } =
            Hexadecimal::from_str("_").unwrap_err()
        {
            assert_eq!(idx, 0);
            assert_eq!(string, "_");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hexadecimal_from_str_number_without_underscore() {
        assert_eq!(
            Hexadecimal::from_str("0x1234567890ABCDEFabcdef").unwrap(),
            Hexadecimal {
                sign: Sign::Empty,
                hexnum: "1234567890ABCDEFabcdef".to_string(),
            }
        )
    }

    #[test]
    fn hexadecimal_from_str_positive_only() {
        if let Error::MalformedPattern(s) =
            Hexadecimal::from_str("+").unwrap_err()
        {
            assert_eq!(s, "+");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hexadecimal_from_str_positive_number() {
        assert_eq!(
            Hexadecimal::from_str("+0x123").unwrap(),
            Hexadecimal {
                sign: Sign::Positive,
                hexnum: "123".to_string(),
            },
        )
    }

    #[test]
    fn hexadecimal_from_str_long_negative_number() {
        assert_eq!(
            Hexadecimal::from_str("-0x1111_2222_3333_4444_5555_6666_7777_8888_9999_0000_aaaa_bbbb_cccc_dddd_eeee_ffff").unwrap(),
            Hexadecimal {
                sign: Sign::Negative,
                hexnum: "1111_2222_3333_4444_5555_6666_7777_8888_9999_0000_aaaa_bbbb_cccc_dddd_eeee_ffff".to_string(),
            },
        )
    }

    #[test]
    fn hexadecimal_from_str_number_ends_with_underscore() {
        if let Error::InvalidChar { idx, string } =
            Decimal::from_str("0x123_").unwrap_err()
        {
            assert_eq!(idx, 5);
            assert_eq!(string, "0x123_");
        } else {
            assert!(false);
        }
    }
}
