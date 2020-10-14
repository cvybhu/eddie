use super::{u256mod25519, TWO_TO_255_MINUS_19};
use super::super::bignum_u512::u512;

/*
    This module implements finding square root modulo 2^255 - 19
    This is not included in u256mod because implementation is more complicated for any modulus
*/


// Calculates euler's criterion
// https://en.wikipedia.org/wiki/Euler%27s_criterion
pub fn has_sqare_root_mod25519(value: u256mod25519) -> bool {
    if value == u256mod25519::zero() {
        return true;
    }

    let criterion_value = value.to_power((TWO_TO_255_MINUS_19 - u512::one()) / u512::from(2));

    if criterion_value == u256mod25519::one() {
        return true;
    } else if criterion_value == u256mod25519::minus_one() {
        return false;
    } else {
        panic!("Euler's criterion didn't work!");
    }
}

const MINUS_ONE_SQRT: u256mod25519 = u256mod25519 { value: u512 { data: [14190309331451158704, 3405592160176694392, 3120150775007532967, 3135389899092516619, 0, 0, 0, 0] },
                                                    this_is_stupid_why: std::marker::PhantomData };

pub fn sqare_root(value: u256mod25519) -> Option<u256mod25519> {
    if !has_sqare_root_mod25519(value) {
        return None;
    }

    // (2^255 - 19) % 8 == 5
    assert_eq!(TWO_TO_255_MINUS_19 % u512::from(8), u512::from(5));

    // That means that we can apply solution by Legrende
    // https://en.wikipedia.org/wiki/Quadratic_residue#cite_ref-32
    let result = value.to_power((TWO_TO_255_MINUS_19 + u512::from(3)) / u512::from(8));

    let result_sqared = result * result;
    if result_sqared == value {
        return Some(result);
    }

    if result_sqared == -value {
        return Some(MINUS_ONE_SQRT * result);
    }

    panic!("sqare_root mod 25519 didnt work!");
}


#[cfg(test)]
mod tests {
    use super::super::{u256mod25519, CURVE_D_CONSTANT};
    use super::super::super::bignum_u512::u512;
    use super::*;
    use std::str::FromStr;
    use crate::test_utils::TEST_VALUES;

    #[test]
    fn basic_squares() {

        // 0^2 == 0
        assert_eq!(sqare_root(u256mod25519::zero()), Some(u256mod25519::zero()));
        
        // 1^2 == 1
        assert_eq!(sqare_root(u256mod25519::zero()), Some(u256mod25519::zero()));

        let minus_one_sqrt = u256mod25519::from(u512::from_str("19681161376707505956807079304988542015446066515923890162744021073123829784752").unwrap());
        assert_eq!(minus_one_sqrt * minus_one_sqrt, u256mod25519::minus_one());
        assert_eq!(MINUS_ONE_SQRT, minus_one_sqrt);

        // 2^2 == 4
        let sqrt4 = sqare_root(u256mod25519::from(4)).unwrap();
        assert_eq!(sqrt4 * sqrt4, u256mod25519::from(4));

        // 13371337 has no square root (Some random internet calculator told me so)
        assert_eq!(sqare_root(u256mod25519::from(13371337)), None);

        // Assure that dy^2 + 1 can not be 0
        // dy^2 + 1 == 0   <=>  y^2 == -1*d^-1
        let minus_d_inverse = -(CURVE_D_CONSTANT.mul_inverse());
        assert_eq!(sqare_root(minus_d_inverse), None);

        {
            // Assure that 1 - dxxyy != 0
            let one_plus_d = u256mod25519::one() + CURVE_D_CONSTANT;
            let delta_sqrt = sqare_root(u256mod25519::from(4) + one_plus_d * one_plus_d).unwrap();
            
            let solution_1 = (one_plus_d - delta_sqrt) / u256mod25519::from(2);
            assert_eq!(sqare_root(solution_1), None);
    
            let solution_2 = (one_plus_d + delta_sqrt) / u256mod25519::from(2);
            assert_eq!(sqare_root(solution_2), None);
        }

        {
            // Assure that 1 + dxxyy != 0
            let one_min_d = u256mod25519::one() - CURVE_D_CONSTANT;
            let delta_sqrt: u256mod25519 = sqare_root(u256mod25519::from(4) + one_min_d * one_min_d).unwrap();
            
            let solution_1 = (one_min_d - delta_sqrt) / u256mod25519::from(2);
            assert_eq!(sqare_root(solution_1), None);
    
            let solution_2 = (one_min_d + delta_sqrt) / u256mod25519::from(2);
            assert_eq!(sqare_root(solution_2), None);
        }
    }

    #[test]
    fn sqrt_of_test_values() {
        for test_val in TEST_VALUES {
            if let Some(sqare_root_val) = sqare_root(u256mod25519::from(test_val)) {
                assert_eq!(sqare_root_val * sqare_root_val, u256mod25519::from(test_val));
            }
        }
    }
}
