#![allow(non_camel_case_types)]
use super::bignum_u512::u512;

/*
    u256mod represents unsigned 256-bit integer modulo some number
    All operations are done % M::modulus();
    template type M has function modulus() which returns modulo for this class
    Will be changed to const u512 once possible
*/
#[derive(Clone, Copy, Debug)]
pub struct u256mod<M: ModulusTrait> {
    // Struct elements are public only because this is the only way to create constants
    // You should always create u256mod<M> using from::()
    pub value: u512,
    pub this_is_stupid_why: std::marker::PhantomData<M>
}

pub trait ModulusTrait: Clone + Copy + std::fmt::Debug { 
    // Returns modulus to be used for operations
    fn modulus() -> u512;

    // Returns euler's totient of modulus (number of smaller coprime numbers)
    // Needed for inversion (too lazy to make signed integers for extended euclid)
    fn modulus_phi() -> u512;
}

impl<M: ModulusTrait> u256mod<M> {
    pub fn zero() -> u256mod<M> {
        return u256mod::from(0);
    }

    pub fn one()  -> u256mod<M> {
        return u256mod::from(1);
    }

    pub fn minus_one() -> u256mod<M> {
        return u256mod::from(-1);
    }

    pub fn to_u512(&self) -> u512 {
        return self.value;
    }

    // Gets multiplicative inverse for u256mod
    // u256mod * u256mod.mul_inverse() == 1
    // If none is found panics
    pub fn mul_inverse(&self) -> u256mod<M> {
        return self.try_mul_inverse().expect("Multiplicative inverse of u256mod not found!");
    }

    // Tries getting multiplicative inverse for u256mod
    pub fn try_mul_inverse(&self) -> Option<u256mod<M>> {
        
        if u512::gcd(self.value, M::modulus()) != u512::one() {
            return None;
        }

        let result = self.to_power(M::modulus_phi() - u512::one());
        //assert_eq!(self * &result, u256mod::one());

        return Some(result);
    }

    pub fn to_power(&self, mut p: u512) -> u256mod<M> {
        let mut result = u256mod::one();
        let mut cur_self_power = *self;

        while p != u512::zero() {
            if p.get_bit(0) {
                result *= cur_self_power;
            }

            cur_self_power *= cur_self_power;
            p >>= 1;
        }

        return result;
    }
}

// All from::(...) perform modulo on creation
mod from; 
mod compare;
mod add;
mod substract;
mod multiply;
mod divide;

#[cfg(test)]
mod test_modulus;

impl<M: ModulusTrait> std::fmt::Display for u256mod<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::{u256mod, ModulusTrait};
    use super::test_modulus::TestModulus;
    use super::super::bignum_u512::u512;
    use std::str::FromStr;
    use crate::test_utils::TEST_VALUES;

    type u256modTest = u256mod<TestModulus>;

    #[test]
    fn basic() {
        assert_eq!(u256modTest::zero().value, u512::zero());
        assert_eq!(u256modTest::zero().to_u512(), u512::zero());

        assert_eq!(u256modTest::one().value, u512::one());
        assert_eq!(u256modTest::zero().to_u512(), u512::zero());

        assert_eq!(u256modTest::minus_one().value, TestModulus::modulus() - u512::one());
        assert_eq!(u256modTest::minus_one().to_u512(), TestModulus::modulus() - u512::one());
    }

    #[test]
    fn power() {
        assert_eq!(u256modTest::one().to_power(u512::max_value()), u256modTest::one());
        assert_eq!(u256modTest::from(2).to_power(u512::from(128)), u256mod::from(u512::power_of_2(128)));

        assert_eq!(u256modTest::from(123456).to_power(u512::zero()), u256modTest::one());

        assert_eq!(u256modTest::from(u512::from(284632784962398737432432_u128)).to_power(u512::from(3743)).to_u512(),
                   u512::from_str("41166130956757411214395459657692725847126926398338758101005456736948").unwrap());

        // In python:
        // (284632784962398737432432**3743) % (4 * (2**224 - 2**96 + 1)) == 41166130956757411214395459657692725847126926398338758101005456736948
    }

    #[test]
    fn inverse() {
        assert_eq!(u256modTest::zero().try_mul_inverse(), None);
        assert_eq!(u256modTest::one().mul_inverse(), u256modTest::one());
        assert_eq!(u256modTest::minus_one().mul_inverse(), u256modTest::minus_one());

        assert_eq!(u256modTest::from(4).try_mul_inverse(), None);

        for test_value in TEST_VALUES {
            // if test_value % 2 != 0 && test_value % 4 != 0  => gcd(test_value, 4) == 1
            if test_value % 2 != 0 && test_value % 4 != 0 {
                let cur = u256modTest::from(test_value);
                let cur_inverse = cur.mul_inverse();

                assert_eq!(cur * cur_inverse, u256mod::one());
            }
        }
    }
}