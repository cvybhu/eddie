use super::super::u256_mod::ModulusTrait;
use super::super::bignum_u512::u512;
use super::{TWO_TO_255_MINUS_19, PRIME_GROUP_ORDER, CURVE_GROUP_ORDER};

#[derive(Clone, Copy, Debug)]
pub struct Modulus25519 {}
impl ModulusTrait for Modulus25519 {
    fn modulus() -> u512 {
        return TWO_TO_255_MINUS_19;
    } 

    fn modulus_phi() -> u512 {
        return TWO_TO_255_MINUS_19 - u512::one();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModulusPrimeOrder {}
impl ModulusTrait for ModulusPrimeOrder {
    fn modulus() -> u512 {
        return PRIME_GROUP_ORDER;
    }
    
    fn modulus_phi() -> u512 {
        return PRIME_GROUP_ORDER - u512::one();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModulusCurveOrder {}
impl ModulusTrait for ModulusCurveOrder {
    fn modulus() -> u512 {
        return CURVE_GROUP_ORDER;
    }

    fn modulus_phi() -> u512 {
        return (PRIME_GROUP_ORDER - u512::one()) * u512::from(4);
    }
}
