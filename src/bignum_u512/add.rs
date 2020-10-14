use super::u512;

// Addition

impl std::ops::Add<&u512> for &u512 {
    type Output = u512;

    fn add(self, other: &u512) -> u512 {
        let mut result = u512::zero();
        let mut accumulator: u128 = 0;

        for i in 0..8 {
            accumulator += self.data[i] as u128;
            accumulator += other.data[i] as u128;

            result.data[i] = accumulator as u64;
            accumulator >>= 64;
        }

        if accumulator != 0 {
            panic!("u512 addition overflow!");
        }

        return result;
    }
}

impl std::ops::AddAssign<&u512> for u512 {
    fn add_assign(&mut self, other: &u512) {
        *self = &*self + other;
    }
}

// Different versions for references // TODO - do i have to do mutable references also ;_;
impl std::ops::Add for u512        { type Output = u512; fn add(self, other: u512) -> u512 { return &self + &other; } }
impl std::ops::Add<&u512> for u512 { type Output = u512; fn add(self, other: &u512) -> u512 { return &self + other; } }
impl std::ops::Add<u512> for &u512 { type Output = u512; fn add(self, other: u512) -> u512 { return self + &other; } }
impl std::ops::AddAssign<u512> for u512 { fn add_assign(&mut self, other: u512) { *self = &*self + &other; } }

#[cfg(test)]
mod tests {
    use super::super::u512;
    use crate::test_utils::TEST_VALUES;

    #[test]
    fn add() {
        let zero: u512 = u512::zero();
        let one: u512 = u512::one();
        let two: u512 = u512::from(2);

        assert_eq!(&zero + &zero, zero); // 0 + 0 == 0
        assert_eq!(&zero + &one, one);   // 0 + 1 == 1
        assert_eq!(&one + &zero, one);   // 1 + 0 == 1
        assert_eq!(&one + &one, two);    // 1 + 1 == 2

        for test_value1 in TEST_VALUES {
            for test_value2 in TEST_VALUES {
                
                // test_value1 + test_value2 > u128::max_value() - avoid overflow
                if *test_value1 > u128::max_value() - *test_value2 {
                    continue;
                }

                let sum_u128: u128 = test_value1 + test_value2;
                let sum_u512: u512 = u512::from(*test_value1) + u512::from(*test_value2);
                assert_eq!(sum_u512, u512::from(sum_u128));

                let plus_one: u512 = u512::from(*test_value1) + u512::one();
                assert_eq!(plus_one, u512::from(test_value1 + 1));
            }
        }
    }
}