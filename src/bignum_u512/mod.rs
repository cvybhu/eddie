#![allow(non_camel_case_types)]
/*
    u512 is an 512-bit unsigned integer
    can be used like u32, u64
*/

#[derive(Clone, Copy, Debug, Hash)]
pub struct u512 {
    pub data: [u64; 8]
    // u512 = data[0]*2^0 + data[1]*2^64 + data[2]*2^128 + data[3]*2^192 +
    //      + data[4]*2^256 + data[5]*2^320 + data[6]*2^384 + data[7]*2^448
}

impl u512 {
    pub const fn zero() -> u512 { u512 { data: [0; 8] }}
    pub const fn one() ->  u512 { u512 { data: [1, 0, 0, 0, 0, 0, 0, 0] } }

    pub const fn max_value() -> u512 { u512 { data: [u64::max_value(); 8] } }

    pub fn power_of_2(power_of_2: u32) -> u512 { &u512::one() << power_of_2 }

    pub const fn to_u8(&self) -> u8     { self.data[0] as u8 }
    pub const fn to_u16(&self) -> u16   { self.data[0] as u16 }
    pub const fn to_u32(&self) -> u32   { self.data[0] as u32 }
    pub const fn to_u64(&self) -> u64   { self.data[0] as u64 }
    pub const fn to_u128(&self) -> u128 { (self.data[0] as u128) + (1u128 << 64) * (self.data[1] as u128) }

    pub fn get_bit(&self, bit_index: usize) -> bool {
        let bit_u64: u64 = self.data[bit_index/64];
        let shifted: u64 = bit_u64 >> (bit_index % 64);
        
        return if (shifted & 1u64) == 1u64 { true } else { false };
    }

    pub fn set_bit(&mut self, bit_index: usize, bit_value: bool) {
        let bit_u64: &mut u64 = &mut self.data[bit_index/64];
        let shifted_one: u64 = 1u64 << (bit_index % 64);

        if bit_value == true {
            *bit_u64 |= shifted_one;
        } else {
            *bit_u64 &= !shifted_one;
        }
    }
}

mod from;
mod compare;
mod bitshift;
mod add;
mod substract;
mod multiply;
mod divide;
mod string_ops;
mod gcd;

// Tests

#[cfg(test)]
mod tests {
    use super::u512;

    #[test]
    fn zero() {
        let zero: u512 = u512::zero();
        assert_eq!(zero.data, [0; 8]);
    }

    #[test]
    fn one() {
        let one: u512 = u512::one();
        assert_eq!(one.data, [1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(one, u512::from(1));
    }

    #[test]
    fn from() {
        for i in 0..=32 {
            let cur_u128 = if i == 32 { u128::max_value() } else { u128::max_value() / 32 * i };
            let cur_u512 = u512::from(cur_u128);

            assert_eq!(cur_u512.data[0], cur_u128 as u64);
            assert_eq!(cur_u512.data[1], (cur_u128 >> 64) as u64);
            let converted_to_u128: u128 = cur_u512.data[0] as u128 + (cur_u512.data[1] as u128) * (u64::max_value() as u128 + 1);
            assert_eq!(converted_to_u128, cur_u128);

            if i == 32 {
                assert_eq!(cur_u128, u128::max_value());
            }
        }
    }

    #[test]
    fn get_bit() {
        // Basic values
        let zero = u512::zero();
        let one = u512::one();
        let max_value = u512::max_value();

        for bit_index in 0..512 {
            assert_eq!(zero.get_bit(bit_index), false);
            assert_eq!(max_value.get_bit(bit_index), true);
            assert_eq!(one.get_bit(bit_index), if bit_index == 0 {true} else {false} );
        }

        // Single bit on u128
        for on_index in 0..128 {
            let cur_u128: u128 = 1 << on_index;

            let cur_u512 = u512::from(cur_u128);
            for bit_index in 0..512 {
                assert_eq!(cur_u512.get_bit(bit_index), if bit_index == on_index {true} else {false});
            }
        }
    }

    #[test]
    fn set_bit() {
        // 0 -> 1 -> 0
        let mut zero_to_one = u512::zero();

        zero_to_one.set_bit(0, true);
        assert_eq!(zero_to_one, u512::one());

        zero_to_one.set_bit(0, false);
        assert_eq!(zero_to_one, u512::zero());

        // Setting single bit
        for on_index in 0..512 {
            let mut cur_u512: u512 = u512::zero();
            cur_u512.set_bit(on_index, true);

            for bit_index in 0..512 {
                assert_eq!(cur_u512.get_bit(bit_index), if bit_index == on_index {true} else {false});
            }
            
            if on_index < 128 {
                let cur_u128: u128 = 1 << on_index;
                assert_eq!(cur_u512, u512::from(cur_u128));
            }

            cur_u512.set_bit(on_index, false);
            assert_eq!(cur_u512, u512::zero());
        }
    }
}
