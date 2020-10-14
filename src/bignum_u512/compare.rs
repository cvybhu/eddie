// Comparison - equality and ordering
use super::u512;

// Equality
impl std::cmp::PartialEq for u512 {
    fn eq(&self, other: &u512) -> bool {
        return self.data == other.data;
    }
}

impl std::cmp::Eq for u512 {}


// Ordering
impl Ord for u512 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in (0..8).rev() {
            if self.data[i] < other.data[i] {
                return std::cmp::Ordering::Less;
            }

            if self.data[i] > other.data[i] {
                return std::cmp::Ordering::Greater;
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

impl std::cmp::PartialOrd for u512 {
    fn partial_cmp(&self, other: &u512) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}


#[cfg(test)]
mod tests {
    use super::super::u512;

    #[test]
    fn basic_equality() {
        let zero = u512::zero();
        let one = u512::one();

        assert!(zero == zero);
        assert!(one == one);
        assert!(zero != one);
        assert!(one != zero);
        assert!(u512::max_value() != u512::zero());
    }
}