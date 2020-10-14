use super::u512;

impl std::ops::Mul for &u512 {
    type Output = u512;

    fn mul(self, other: &u512) -> u512 {
        let mut result = u512::zero();
    
        // self * other = sum i, j in 0..8 self.data[i] * other.data[j] * 2^((i+j) * 64)
        for i in 0..8 {
            for j in 0..8 {
                let cur_product: u128 = (self.data[i as usize] as u128) * (other.data[j as usize] as u128);
                
                if cur_product == 0 {
                    continue;
                }

                if i + j > 7 {
                    panic!("u512 multiplication overflow!");
                }

                // Add cur_product * 2^((i+j) * 64) to result
                let mut accumulator: u128 = cur_product;
                for res_idx in (i + j)..8 {
                    accumulator += result.data[res_idx] as u128;
                    result.data[res_idx] = accumulator as u64;
                    accumulator >>= 64;

                    if accumulator == 0 {
                        break;
                    }
                }

                if accumulator != 0 {
                    panic!("u512 multiplication overflow!");
                }
            }
        }

        return result;
    }
}

impl std::ops::MulAssign<&u512> for u512 {
    fn mul_assign(&mut self, other: &u512) {
        *self = &*self * other;
    }
}

impl std::ops::Mul for u512        { type Output = u512; fn mul(self, other: u512) -> u512 { return &self * &other; } }
impl std::ops::Mul<&u512> for u512 { type Output = u512; fn mul(self, other: &u512) -> u512 { return &self * other; } }
impl std::ops::Mul<u512> for &u512 { type Output = u512; fn mul(self, other: u512) -> u512 { return self * &other; } }
impl std::ops::MulAssign<u512> for u512 { fn mul_assign(&mut self, other: u512) { *self = &*self * &other; } }

#[cfg(test)]
mod tests {
    use super::super::u512;
    use crate::test_utils::TEST_VALUES;

    #[test]
    fn multiply() {
        let zero: u512 = u512::zero();
        let one: u512 = u512::one();

        assert_eq!(&zero * &zero, zero); // 0 * 0 == 0
        assert_eq!(&zero * &one, zero);  // 0 * 1 == 0
        assert_eq!(&one * &zero, zero);  // 1 * 0 == 0
        assert_eq!(&one * &one, one);    // 1 * 1 == 1

        for test_val1 in TEST_VALUES {
            for test_val2 in TEST_VALUES {

                if *test_val1 > u128::max_value() / *test_val2 {
                    continue;
                }

                let mul_u128: u128 = test_val1 * test_val2;
                let mul_u512: u512 = &u512::from(*test_val1) * &u512::from(*test_val2);
                assert_eq!(mul_u512, u512::from(mul_u128));
            }
        }
    }
}