/*
    struct TestModulus is used as template parameter for testing u256mod<M>
*/
use super::ModulusTrait;
use super::super::bignum_u512::u512;

// TEST_MODULUS == 4 * (2^224 - 2^96 + 1)
// 2^224 - 2^96 + 1 is a prime number
// TEST_MODULUS is not prime to check inverse cases
// TODO - is this prime number patented?
pub const TEST_MODULUS: u512 = u512 { data: [4, 18446744056529682432, 18446744073709551615, 17179869183, 0, 0, 0, 0] };

#[derive(Clone, Copy, Debug)]
pub struct TestModulus {}
impl ModulusTrait for TestModulus {
    fn modulus() -> u512 {
        return TEST_MODULUS;
    }

    fn modulus_phi() -> u512 {
        // phi(2^224 - 2^96 + 1) * phi(4)
        return (u512::power_of_2(224) - u512::power_of_2(96)) * u512::from(2);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::bignum_u512::u512;
    use super::super::ModulusTrait;
    use super::{TEST_MODULUS, TestModulus};
    use std::str::FromStr;

    #[test]
    fn constant_value() {
        let desired_modulus_val: u512 = u512::from(4) * (u512::power_of_2(224) - u512::power_of_2(96) + u512::one());

        assert_eq!(TEST_MODULUS, desired_modulus_val);
        assert_eq!(TEST_MODULUS, u512::from_str("107839786668602559178668060348078522694231665040105232574040265195524").unwrap());
        assert_eq!(&TEST_MODULUS.to_string(), "107839786668602559178668060348078522694231665040105232574040265195524");
    }

    #[test]
    fn modulus_trait() {
        let desired_modulus_val: u512 = u512::from(4) * (u512::power_of_2(224) - u512::power_of_2(96) + u512::one());
        let desired_modulus_phi_val: u512 = u512::from(2) * (u512::power_of_2(224) - u512::power_of_2(96));

        assert_eq!(TestModulus::modulus(), TEST_MODULUS);
        assert_eq!(TestModulus::modulus(), desired_modulus_val);
        assert_eq!(TestModulus::modulus(), u512::from_str("107839786668602559178668060348078522694231665040105232574040265195524").unwrap());
        assert_eq!(&TestModulus::modulus().to_string(), "107839786668602559178668060348078522694231665040105232574040265195524");

        assert_eq!(TestModulus::modulus_phi(), desired_modulus_phi_val);
        assert_eq!(TestModulus::modulus_phi(), u512::from_str("53919893334301279589334030174039261347115832520052616287020132597760").unwrap());
        assert_eq!(&TestModulus::modulus_phi().to_string(), "53919893334301279589334030174039261347115832520052616287020132597760");
    }
}
