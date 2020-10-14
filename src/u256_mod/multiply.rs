use super::{u256mod, ModulusTrait};

// Multiplication
impl<M: ModulusTrait> std::ops::Mul for &u256mod<M> {
    type Output = u256mod<M>;

    fn mul(self, other: &u256mod<M>) -> u256mod<M> {
        return u256mod::from(self.value * other.value);
    }
}

impl<M: ModulusTrait> std::ops::MulAssign<&u256mod<M>> for u256mod<M> {
    fn mul_assign(&mut self, other: &u256mod<M>) {
        *self = &*self * other;
    }
}

// Versions with different reference combinations
impl<M: ModulusTrait> std::ops::Mul for u256mod<M> {
    type Output = u256mod<M>;
    fn mul(self, other: u256mod<M>) -> u256mod<M> { return &self * &other; }
}

impl<M: ModulusTrait> std::ops::Mul<&u256mod<M>> for u256mod<M> {
    type Output = u256mod<M>;
    fn mul(self, other: &u256mod<M>) -> u256mod<M> { return &self * other; }
}

impl<M: ModulusTrait> std::ops::Mul<u256mod<M>> for &u256mod<M> {
    type Output = u256mod<M>;
    fn mul(self, other: u256mod<M>) -> u256mod<M> { return self * &other; }
}

impl<M: ModulusTrait> std::ops::MulAssign<u256mod<M>> for u256mod<M> {
    fn mul_assign(&mut self, other: u256mod<M>) {
        *self = &*self * &other;
    }
}