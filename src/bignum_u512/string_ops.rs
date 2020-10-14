// Handles conversion from/to string
use super::u512;

impl std::str::FromStr for u512 {
    type Err = &'static str;

    fn from_str(number_str: &str) -> Result<u512, &'static str> {
        let mut result = u512::zero();

        for digit_char in number_str.chars() {
            let cur_digit: u32 = match digit_char.to_digit(10) {
                None => return Err("Bad digit"),
                Some(digit) => digit
            };

            result = &result * &u512::from(10) + &u512::from(cur_digit as u64);
        }

        return Ok(result);
    }
}

// It cant be std::str::ToString because Display causes ToString
impl u512 {
    fn to_string(&self) -> String {
        if *self == u512::zero() {
            return String::from("0");
        }

        let mut result = String::from("");

        let mut self_copy: u512 = *self;
        while self_copy != u512::zero() {
            let cur_digit: u64 = (&self_copy % &u512::from(10)).to_u64();
            result = cur_digit.to_string() + &result;
            self_copy = &self_copy / &u512::from(10);
        }

        return result;
    }
}

impl std::fmt::Display for u512 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::super::u512;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(Ok(u512::from(0)), u512::from_str("0"));
        assert_eq!(Ok(u512::from(123412)), u512::from_str("123412"));
        
        let assert_err = | result | { if let Ok(_) = result { panic!("Ok should be error!"); }};
        assert_err(u512::from_str("non digit"));

        assert_eq!(Ok(u512::from(0)), u512::from_str("0"));

        let max_value: u512 = u512::from_str("13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095").unwrap();
        assert_eq!(&max_value, &u512::max_value());
    }
}    