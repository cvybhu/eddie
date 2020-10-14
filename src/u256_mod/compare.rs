use super::{u256mod, ModulusTrait};

// Comparison - equality and ordering

// Equality
impl<M: ModulusTrait> std::cmp::PartialEq for u256mod<M> {
    fn eq(&self, other: &u256mod<M>) -> bool {
        return self.value == other.value;
    }
}

impl<M: ModulusTrait> std::cmp::Eq for u256mod<M> {}


// Ordering
impl<M: ModulusTrait> Ord for u256mod<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.value.cmp(&other.value);
    }
}

impl<M: ModulusTrait> std::cmp::PartialOrd for u256mod<M> {
    fn partial_cmp(&self, other: &u256mod<M>) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

#[cfg(test)]
mod tests {
    
}

