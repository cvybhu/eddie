use super::{u256mod, ModulusTrait};

// Division
impl<M: ModulusTrait> std::ops::Div for &u256mod<M> {
    type Output = u256mod<M>;

    fn div(self, other: &u256mod<M>) -> u256mod<M> {
        return self * other.mul_inverse();
    }
}

// Versions with different reference combinations
impl<M: ModulusTrait> std::ops::Div for u256mod<M> {
    type Output = u256mod<M>;
    fn div(self, other: u256mod<M>) -> u256mod<M> { return &self / &other; }
}

impl<M: ModulusTrait> std::ops::Div<&u256mod<M>> for u256mod<M> {
    type Output = u256mod<M>;
    fn div(self, other: &u256mod<M>) -> u256mod<M> { return &self / other; }
}

impl<M: ModulusTrait> std::ops::Div<u256mod<M>> for &u256mod<M> {
    type Output = u256mod<M>;
    fn div(self, other: u256mod<M>) -> u256mod<M> { return self / &other; }
}