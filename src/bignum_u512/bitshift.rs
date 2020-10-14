use super::u512;

// Left shift
impl std::ops::Shl<u32> for &u512 {
    type Output = u512;

    fn shl(self, shift: u32) -> u512 {
        let mut result: u512 = u512::zero();

        for bit_num in shift..512 {
            result.set_bit(bit_num as usize, self.get_bit((bit_num - shift) as usize));
        }

        return result;
    }
}

impl std::ops::Shl<u32> for u512 { type Output = u512; fn shl(self, shift: u32) -> u512 { &self << shift }}


impl std::ops::ShlAssign<u32> for u512 {
    fn shl_assign(&mut self, shift: u32) {

        if shift != 1 {
            *self = *self << shift;
        }

        // Fast implementation for shift == 1
        // It is needed for acceptable division speed
        let two_to_63: u64 = 1u64 << 63;

        self.data[7] <<= 1;
        for i in (0..7).rev() {
            if self.data[i] & two_to_63 == two_to_63 {
                self.data[i + 1] |= 1u64;
            }

            self.data[i] <<= 1;
        }
    }
}

// Right shift
impl std::ops::Shr<u32> for &u512 {
    type Output = u512;

    fn shr(self, shift: u32) -> u512 {
        let mut result: u512 = u512::zero();

        for bit_num in shift..512 {
            result.set_bit((bit_num - shift) as usize, self.get_bit(bit_num as usize));
        }

        return result;
    }
}

impl std::ops::ShrAssign<u32> for u512 {
    fn shr_assign(&mut self, shift: u32) {
        *self = *self >> shift;
    }
}

impl std::ops::Shr<u32> for u512 { type Output = u512; fn shr(self, shift: u32) -> u512 { &self >> shift }}

#[cfg(test)]
mod tests {
    

}