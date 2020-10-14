use super::{u256mod, ModulusTrait};
use super::super::bignum_u512::u512;

// Creation from unsigned integer values
// All types other that &u512 are handled by converting to &u512

impl<M: ModulusTrait> From<&u512> for u256mod<M> {
    fn from(val: &u512) -> u256mod<M> {
        return u256mod::<M> { 
            value: val % M::modulus(), 
            this_is_stupid_why: std::marker::PhantomData 
        };
    }
}

impl<M: ModulusTrait> From<u8>    for u256mod<M> { fn from(val: u8)    -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<&u8>   for u256mod<M> { fn from(val: &u8)   -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<u16>   for u256mod<M> { fn from(val: u16)   -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<&u16>  for u256mod<M> { fn from(val: &u16)  -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<u32>   for u256mod<M> { fn from(val: u32)   -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<&u32>  for u256mod<M> { fn from(val: &u32)  -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<u64>   for u256mod<M> { fn from(val: u64)   -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<&u64>  for u256mod<M> { fn from(val: &u64)  -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<u128>  for u256mod<M> { fn from(val: u128)  -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<&u128> for u256mod<M> { fn from(val: &u128) -> u256mod<M> { u256mod::from(&u512::from(val)) } }
impl<M: ModulusTrait> From<u512>  for u256mod<M> { fn from(val: u512)  -> u256mod<M> { u256mod::from(&u512::from(val)) } }

// Creation from integer values
// All types other are handled by converting to i128

impl<M: ModulusTrait> From<i128> for u256mod<M> {
    fn from(val: i128) -> u256mod<M> {
        if val >= 0 {
            return u256mod::from(val as u128);
        }

        // if val < 0 then calculate (0 - |val|) % M
        // TODO - does this work when |val| > M ?  - this shouldnt matter for Ed25519 probably
        // For now lets panic!
        let positive_val = u512::from(-val);
        if positive_val >= M::modulus() {
            panic!("This has not been implemented yet, negative values must be smaller than modulus");
        }
        
        return u256mod::from(u512::zero()) - u256mod::from(positive_val);
    }
}

impl<M: ModulusTrait> From<i8>    for u256mod<M> { fn from(val: i8)    -> u256mod<M> { u256mod::from(val as i128)  }}
impl<M: ModulusTrait> From<&i8>   for u256mod<M> { fn from(val: &i8)   -> u256mod<M> { u256mod::from(*val as i128) }}
impl<M: ModulusTrait> From<i16>   for u256mod<M> { fn from(val: i16)   -> u256mod<M> { u256mod::from(val as i128)  }}
impl<M: ModulusTrait> From<&i16>  for u256mod<M> { fn from(val: &i16)  -> u256mod<M> { u256mod::from(*val as i128) }}
impl<M: ModulusTrait> From<i32>   for u256mod<M> { fn from(val: i32)   -> u256mod<M> { u256mod::from(val as i128)  }}
impl<M: ModulusTrait> From<&i32>  for u256mod<M> { fn from(val: &i32)  -> u256mod<M> { u256mod::from(*val as i128) }}
impl<M: ModulusTrait> From<i64>   for u256mod<M> { fn from(val: i64)   -> u256mod<M> { u256mod::from(val as i128)  }}
impl<M: ModulusTrait> From<&i64>  for u256mod<M> { fn from(val: &i64)  -> u256mod<M> { u256mod::from(*val as i128) }}
impl<M: ModulusTrait> From<&i128> for u256mod<M> { fn from(val: &i128) -> u256mod<M> { u256mod::from(*val as i128) }}