use super::{u256mod, ModulusTrait};

// Substraction
impl<M: ModulusTrait> std::ops::Sub for &u256mod<M> {
    type Output = u256mod<M>;

    fn sub(self, other: &u256mod<M>) -> u256mod<M> {

        let result_value =  if self.value >= other.value {
                                &self.value - &other.value
                            } else {
                                M::modulus() + &self.value - &other.value
                            };
        
        return u256mod { value: result_value, this_is_stupid_why: std::marker::PhantomData };
    }
}

impl<M: ModulusTrait> std::ops::SubAssign<&u256mod<M>> for u256mod<M> {
    fn sub_assign(&mut self, other: &u256mod<M>) {
        *self = *self - other;
    }
}

// Negation
impl<M: ModulusTrait> std::ops::Neg for &u256mod<M> {
    type Output = u256mod<M>;

    fn neg(self) -> u256mod<M> {
        return u256mod::zero() - self;
    }
}

// Versions with different reference combinations
impl<M: ModulusTrait> std::ops::Sub for u256mod<M> {
    type Output = u256mod<M>;
    fn sub(self, other: u256mod<M>) -> u256mod<M> { return &self - &other; }
}

impl<M: ModulusTrait> std::ops::Sub<&u256mod<M>> for u256mod<M> {
    type Output = u256mod<M>;
    fn sub(self, other: &u256mod<M>) -> u256mod<M> { return &self - other; }
}

impl<M: ModulusTrait> std::ops::Sub<u256mod<M>> for &u256mod<M> {
    type Output = u256mod<M>;
    fn sub(self, other: u256mod<M>) -> u256mod<M> { return self - &other; }
}
impl<M: ModulusTrait> std::ops::SubAssign for u256mod<M> {
    fn sub_assign(&mut self, other: u256mod<M>) {
        *self = *self - other;
    }
}

impl<M: ModulusTrait> std::ops::Neg for u256mod<M> {
    type Output = u256mod<M>;

    fn neg(self) -> u256mod<M> {
        return -&self;
    }
}