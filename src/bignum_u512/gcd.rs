use super::u512;

impl u512 {
    // Finds greates common divisor of a and b using euclidean agorithm
    pub fn gcd(mut a: u512, mut b: u512) -> u512 {
        loop {
            if b == u512::zero() {
                return a;
            }

            a %= b;
            
            if a == u512::zero() {
                return b;
            }

            b %= a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::u512;
    use crate::test_utils::TEST_VALUES;

    // Taken from https://doc.rust-lang.org/std/ops/trait.Div.html
    fn original_gcd(x: u128, y: u128) -> u128 {
        let mut x = x;
        let mut y = y;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    }

    fn recurrent_gcd(x: u128, y: u128) -> u128 {
        if x < y {
            return recurrent_gcd(y, x);
        }

        if y == 0 {
            return x;
        }

        return recurrent_gcd(x % y, y);
    }

    #[test]
    fn gcd() {
        for test_val1 in TEST_VALUES.iter().step_by(6) {
            for test_val2 in TEST_VALUES {
                let gcd_u128 = original_gcd(*test_val1, *test_val2);
                let gcd_u128_rec = recurrent_gcd(*test_val1, *test_val2);
                let gcd_u512 = u512::gcd(u512::from(test_val1), u512::from(test_val2));

                assert_eq!(gcd_u128, gcd_u128_rec);
                assert_eq!(gcd_u512, u512::from(gcd_u128));

                assert_eq!(u512::from(test_val1) % gcd_u512, u512::zero());
                assert_eq!(u512::from(test_val2) % gcd_u512, u512::zero());
            }
        }
    }
}