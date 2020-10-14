use super::u512;

// Substraction
impl std::ops::Sub for &u512 {
    type Output = u512;

    fn sub(self, other: &u512) -> u512 {
        if self < other { 
            panic!("u512 substraction underflow!");
        }
        
        let mut self_copy: u512 = *self;
        let mut result: u512 = u512::zero();

        let mut borrowed = false;
        for i in 0..8 {
            if borrowed {
                if self_copy.data[i] != 0 {
                    self_copy.data[i] -= 1;
                    borrowed = false;
                } else {
                    self_copy.data[i] = u64::max_value(); // 2^64 - 1
                }
            }

            if self_copy.data[i] >= other.data[i] {
                result.data[i] = self_copy.data[i] - other.data[i];
            } else {
                borrowed = true; // Borrow 2^64 from future (2^64 == u64::max_value() + 1)
                result.data[i] = u64::max_value() - other.data[i] + self_copy.data[i] + 1;
            }
        }

        return result;
    }
}

impl std::ops::SubAssign<&u512> for u512 {
    fn sub_assign(&mut self, other: &u512) {
        *self = &*self - other;
    }
}

impl std::ops::Sub for u512        { type Output = u512; fn sub(self, other: u512) -> u512 { return &self - &other; } }
impl std::ops::Sub<&u512> for u512 { type Output = u512; fn sub(self, other: &u512) -> u512 { return &self - other; } }
impl std::ops::Sub<u512> for &u512 { type Output = u512; fn sub(self, other: u512) -> u512 { return self - &other; } }
impl std::ops::SubAssign<u512> for u512 { fn sub_assign(&mut self, other: u512) { *self = &*self - &other; } }

#[cfg(test)]
mod tests {
    use super::super::u512;
    use std::str::FromStr;
    use crate::test_utils::TEST_VALUES;


    #[test]
    fn substract() {
        let zero: u512 = u512::zero();
        let one: u512 = u512::one();

        assert_eq!(&zero - &zero, zero); // 0 + 0 == 0
        assert_eq!(&one - &zero, one);   // 1 - 0 == 1
        assert_eq!(&one - &one, zero);   // 1 - 1 == 0

        for test_value1 in TEST_VALUES {
            for test_value2 in TEST_VALUES {
                for i in 0..2 {
                    let test_value_2: u128 = if i == 0 {*test_value2} else {(*test_value2 as u64) as u128};

                    if test_value1 < &test_value_2 {
                        continue;
                    }
    
                    let u512_sub: u512 = &u512::from(*test_value1) - &u512::from(test_value_2);
                    let u128_sub: u128 = test_value1 - test_value_2;
                    assert_eq!(u512_sub, u512::from(u128_sub));
    
                    let minus_one: u512 = &u512::from(*test_value1) - &u512::one();
                    assert_eq!(minus_one, u512::from(test_value1 - 1));
                }
            }
        }

        let big10: u512 = u512::from_str("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let big99: u512 = u512::from_str("9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999").unwrap();
        assert_eq!(&big10 - &big99, u512::one());

        // (32 + 2^420) - 1337
        let smallbig2: u512 = (&u512::one() << 420) + u512::from(32);
        let leet: u512 = u512::from(1337);
        let expected: u512 = u512::from_str("2707685248164858261307045101702230179137145581421695874189921465443966120903931272499975005961073806735733604454495675614231271").unwrap();
        assert_eq!(&smallbig2 - &leet, expected);
    }
}