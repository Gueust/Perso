use std;

// Price encoded as int with 6 digits.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(i64);

impl Price {
    // Assumes ascii encoding and non-negative prices.
    pub fn parse_str(str: &str) -> Result<Price, String> {
        let mut pre_dot: i64 = 0;
        let mut seen_dot = false;
        let mut post_dot: i64 = 0;
        let mut post_dot_cnt: i64 = 0;
        for c in str.chars() {
            if c == '.' {
                seen_dot = true;
                continue
            }
            if c < '0' || c > '9' {
                Err(format!("unable to parse as price {}", str))?
            }
            let digit = match c.to_digit(10) {
                Some(digit) => digit as i64,
                None => Err(format!("unable to parse as price {}", str))?
            };
            if seen_dot {
                if post_dot_cnt < 6 {
                    post_dot = 10 * post_dot + digit;
                    post_dot_cnt += 1;
                } else {
                    if digit != 0 {
                        Err(format!("unable to parse as price (too many digits) {}", str))?
                    }
                }
            } else {
                pre_dot = 10 * pre_dot + digit;
            }
        }
        for _ in post_dot_cnt..6 {
            post_dot *= 10;
        }
        Ok(Price(pre_dot * 1_000_000 + post_dot))
    }

    fn to_float(&self) -> f64 {
        let &Price(p) = self;
        p as f64 / 1e6
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_float().fmt(f)
    }
}

impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_float().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(Price::parse_str("0"), Ok(Price(0)));
        assert_eq!(Price::parse_str("0.0"), Ok(Price(0)));
        assert_eq!(Price::parse_str("0.000001"), Ok(Price(1)));
        assert_eq!(Price::parse_str("0.00001"), Ok(Price(10)));
        assert_eq!(Price::parse_str("0.000010"), Ok(Price(10)));
        assert_eq!(Price::parse_str("0.00001000"), Ok(Price(10)));
        assert_eq!(Price::parse_str("1"), Ok(Price(1_000_000)));
        assert_eq!(Price::parse_str("1.0"), Ok(Price(1_000_000)));
        assert_eq!(Price::parse_str("1.000000"), Ok(Price(1_000_000)));
        assert_eq!(Price::parse_str("1.000000000"), Ok(Price(1_000_000)));
        assert_eq!(Price::parse_str("42.00300000"), Ok(Price(42_003_000)));
        assert_eq!(Price::parse_str("42.00300100"), Ok(Price(42_003_001)));
    }
}
