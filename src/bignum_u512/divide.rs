// u512 division
use super::u512;
use std::convert::TryInto;

/*
    This is an attempt at Knuth's algorithm D for division.
    The code is terrible but I'm to lazy to refactor it
    It passes tests and is fast enough for now
*/

impl u512 {
    fn divide_with_remainder(&self, other: &u512) -> (u512, u512) /* (result, remainder) */ {
        // self/other
        if other > self {
            return (u512::zero(), *self);
        }

        if *other == u512::zero() {
            panic!("u512 division by 0!");
        }

        let div_by_small = |u: &mut [u64], small: u64| -> [u64; 9] {
            let mut result = [0u64; 9];

            if small == 0 {
                panic!("Small div by 0!");
            }

            let mut cur_rem: u128 = *u.last().unwrap() as u128;
            for i in (0..u.len()).rev() {
                let cur_q = cur_rem / small as u128;
                result[i] = cur_q as u64;
                cur_rem -= cur_q * small as u128;

                if i >= 1 {
                    cur_rem <<= 64;
                    cur_rem += u[i - 1] as u128;
                }
            }

            u.copy_from_slice(&[cur_rem as u64, (cur_rem >> 64) as u64, 0, 0, 0, 0, 0, 0, 0]);

            return result;
        };

        let mul_by_small = |digits: &mut [u64], small: u64| {
            let mut acc: u128 = 0;
            for digit in digits {
                acc += *digit as u128 * small as u128;
                *digit = acc as u64;
                acc >>= 64;
            }

            if acc != 0 { panic!("mul_by_small overflow!"); }
        };

        let shift_left_by = |digits: &mut [u64], shift: u32| {
            assert!(shift < 64);
            if shift == 0 {
                return;
            }

            for i in (0..digits.len()).rev() {
                digits[i] <<= shift;

                if i != 0 {
                    digits[i] |= digits[i - 1]>> (64 - shift);
                }
            }
        };

        let shift_right_by = |digits: &mut [u64], shift: u32| {
            assert!(shift < 64);

            if shift == 0 {
                return;
            };

            for i in 0..digits.len() {
                digits[i] >>= shift;

                if i+1 < digits.len() {
                    digits[i] |= digits[i + 1] << (64 - shift);
                }
            }
        };

        let is_bigger = |a: &[u64], b: &[u64]| -> bool {
            for i in (0..usize::max(a.len(), b.len())).rev() {
                let a_val: u64 = if i < a.len() {a[i]} else {0};
                let b_val: u64 = if i < b.len() {b[i]} else {0};

                match a_val.cmp(&b_val) {
                    std::cmp::Ordering::Less => return false,
                    std::cmp::Ordering::Greater => return true,
                    _ => {}
                };
            }

            return false;
        };

        let subs = |a: &mut [u64], b: &[u64]| {
            
            assert!(a.len() >= b.len());
            
            let mut borrow = false;
            for i in 0..a.len() {
                if borrow {
                    if a[i] != 0 {
                        a[i] -= 1;
                        borrow = false;
                    } else {
                        a[i] = u64::max_value();
                    }
                }

                let sub_val = if i < b.len() {b[i]} else {0};
                if a[i] >= sub_val {
                    a[i] -= sub_val;
                } else {
                    borrow = true;
                    a[i] += u64::max_value() - sub_val + 1;
                }
            }

            assert_eq!(borrow, false);
        };

        let mut u: [u64; 9] = [0u64; 9];
        u[0..8].copy_from_slice(&self.data);
        let m = (0..9).rev().find(|i| u[*i] != 0).unwrap() + 1;

        let mut v: [u64; 8] = other.data;
        let n = (0..8).rev().find(|i| v[*i] != 0).unwrap() + 1;

        if n <= 1 {
            let result_digits: [u64; 9] = div_by_small(&mut u, v[n-1]);

            let mut result = u512::zero();
            result.data.copy_from_slice(&result_digits[..8]);

            let mut raminder = u512::zero();
            raminder.data.copy_from_slice(&u[..8]);

            return (result, raminder);
        }

        
        let norm_shift = v[n-1].leading_zeros();
        shift_left_by(&mut u, norm_shift);
        shift_left_by(&mut v, norm_shift);
        

        assert!(v[n-1] & (1u64 << 63) != 0);

        let two_to_64: u128 = 1u128 << 64;
        let mut result = u512::zero();

        for cur_u_highest_index in (n..=m).rev() {
            let cur_u_lowest_index = cur_u_highest_index - n;
            let cur_u: &mut [u64] = &mut u[cur_u_lowest_index..=cur_u_highest_index];
            assert!(cur_u.len() == n + 1);

            let first_two_added: u128 = cur_u[cur_u.len() - 1] as u128 * two_to_64 + cur_u[cur_u.len() - 2] as u128;
            let mut q_guess: u64 = u128::min(first_two_added / v[n - 1] as u128, u64::max_value() as u128).try_into().unwrap();


            // One of (q_guess, q_guess - 1, q_guess - 2) is the correct q
            // Lets try multiplying q_guess times v and see if it is smaller than cur_u
            let mut vq = [0u64; 9];
            vq[..8].copy_from_slice(&v);
            mul_by_small(&mut vq, q_guess);
            let vq_len: usize = (0..9).rev().find(|i| vq[*i] != 0).unwrap_or(0) + 1;

            // While v * q is bigger than cur_u substract v from v * q
            // This should happen max 2 times
            let mut counter = 0;
            while is_bigger(&vq[..vq_len], cur_u) {
                assert!(counter < 4);
                subs(&mut vq, &v[..n]);
                q_guess -= 1;
                counter += 1;
            }

            // Now q_guess is OK
            result.data[cur_u_lowest_index] = q_guess;

            // Substract from u and find next quotient digit
            subs(cur_u, &vq[..vq_len]);
        }

        let mut rem: u512 = u512 { data: u[..8].try_into().unwrap() };
        shift_right_by(&mut rem.data, norm_shift);

        return (result, rem);
    }
}

impl std::ops::Div for &u512 {
    type Output = u512;

    fn div(self, other: &u512) -> u512 {
        let (result, _remainder) = self.divide_with_remainder(other);
        return result;
    }
}

impl std::ops::DivAssign<&u512> for u512 {
    fn div_assign(&mut self, other: &u512) {
        *self = &*self / other;
    }
}

impl std::ops::Div for u512        { type Output = u512; fn div(self, other: u512) -> u512 { return &self / &other; } }
impl std::ops::Div<&u512> for u512 { type Output = u512; fn div(self, other: &u512) -> u512 { return &self / other; } }
impl std::ops::Div<u512> for &u512 { type Output = u512; fn div(self, other: u512) -> u512 { return self / &other; } }
impl std::ops::DivAssign<u512> for u512 { fn div_assign(&mut self, other: u512) { *self = &*self / &other; } }



impl std::ops::Rem for &u512 {
    type Output = u512;

    fn rem(self, modulus: &u512) -> u512 {
        let (_result, remainder) = self.divide_with_remainder(modulus);
        return remainder;
    }
}

impl std::ops::RemAssign<&u512> for u512 {
    fn rem_assign(&mut self, other: &u512) {
        *self = &*self % other;
    }
}

impl std::ops::Rem for u512        { type Output = u512; fn rem(self, other: u512) -> u512 { return &self % &other; } }
impl std::ops::Rem<&u512> for u512 { type Output = u512; fn rem(self, other: &u512) -> u512 { return &self % other; } }
impl std::ops::Rem<u512> for &u512 { type Output = u512; fn rem(self, other: u512) -> u512 { return self % &other; } }
impl std::ops::RemAssign<u512> for u512 { fn rem_assign(&mut self, other: u512) { *self = &*self % &other; } }

#[cfg(test)]
mod tests {
    use super::super::u512;
    use crate::test_utils::{TEST_VALUES, BIG_TEST_VALUES};

    #[test]
    fn divide() {
        let zero = u512::zero();
        let one = u512::one();

        assert_eq!(zero / one, zero);
        assert_eq!(one / one, one);

        for test_val1 in TEST_VALUES.iter() {
            for test_val2 in TEST_VALUES.iter() {
                let div_u128: u128 = test_val1 / test_val2;
                let div_u512: u512 = u512::from(test_val1) / u512::from(test_val2);

                assert_eq!(div_u512, u512::from(div_u128));
            }
        }
    }

    #[test]
    fn remainder() {
        let zero = u512::zero();
        let one = u512::one();

        assert_eq!(zero % one, zero);
        assert_eq!(one % one, zero);

        for test_val1 in TEST_VALUES.iter() {
            for test_val2 in TEST_VALUES.iter() {
                let remainder_u128: u128 = test_val1 % test_val2;
                let remainder_u512: u512 = u512::from(test_val1) % u512::from(test_val2);

                assert_eq!(remainder_u512, u512::from(remainder_u128));
            }
        }
    }

    #[test]
    fn divide_and_remainder_big() {
        for val_a in BIG_TEST_VALUES {
            for val_b in BIG_TEST_VALUES {
                let a_div_b: u512 = val_a / val_b;
                let a_mod_b: u512 = val_a % val_b;

                let div_check: u512 = a_div_b * val_b;
                let mod_check: u512 = val_a - div_check;
                assert!(div_check <= *val_a);
                assert!(mod_check < *val_b);
                assert_eq!(a_mod_b, mod_check);
            }
        }
    }
}
