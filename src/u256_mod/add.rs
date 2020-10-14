use super::{u256mod, ModulusTrait};

// Addition
impl<M: ModulusTrait> std::ops::Add for &u256mod<M> {
    type Output = u256mod<M>;

    fn add(self, other: &u256mod<M>) -> u256mod<M> {
        let sum_value = self.value + other.value;

        if sum_value >= M::modulus() {
            return u256mod { value: &sum_value - &M::modulus(), this_is_stupid_why: std::marker::PhantomData };
        } else {
            return u256mod { value: sum_value, this_is_stupid_why: std::marker::PhantomData };
        }
    }
}

impl<M: ModulusTrait> std::ops::AddAssign<&u256mod<M>> for u256mod<M> {
    fn add_assign(&mut self, other: &u256mod<M>) {
        *self = *self + other;
    }
}

// Versions with different reference combinations
impl<M: ModulusTrait> std::ops::Add for u256mod<M> {
    type Output = u256mod<M>;
    fn add(self, other: u256mod<M>) -> u256mod<M> { return &self + &other; }
}

impl<M: ModulusTrait> std::ops::Add<&u256mod<M>> for u256mod<M> {
    type Output = u256mod<M>;
    fn add(self, other: &u256mod<M>) -> u256mod<M> { return &self + other; }
}

impl<M: ModulusTrait> std::ops::Add<u256mod<M>> for &u256mod<M> {
    type Output = u256mod<M>;
    fn add(self, other: u256mod<M>) -> u256mod<M> { return self + &other; }
}

impl<M: ModulusTrait> std::ops::AddAssign for u256mod<M> {
    fn add_assign(&mut self, other: u256mod<M>) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_add() {
        
    }
}