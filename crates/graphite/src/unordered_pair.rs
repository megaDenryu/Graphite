use std::hash::{DefaultHasher, Hash, Hasher};

/// 順序を持たない同型値の対。
#[derive(Clone)]
#[derive(Debug)]
pub struct UnorderedPair<T> {
    first: T,
    second: T,
}

impl<T> UnorderedPair<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    pub fn endpoints(&self) -> (&T, &T) {
        (&self.first, &self.second)
    }
}

impl<T: PartialEq> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first && self.second == other.second)
            || (self.first == other.second && self.second == other.first)
    }
}

impl<T: Eq> Eq for UnorderedPair<T> {}

impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut first_hasher = DefaultHasher::new();
        self.first.hash(&mut first_hasher);
        let mut second_hasher = DefaultHasher::new();
        self.second.hash(&mut second_hasher);
        state.write_u64(first_hasher.finish() ^ second_hasher.finish());
    }
}

#[cfg(test)]
mod tests {
    use super::UnorderedPair;
    use std::collections::HashSet;

    #[test]
    fn 等価性とhashは順序を区別しない() {
        let forward = UnorderedPair::new("alice", "bob");
        let reverse = UnorderedPair::new("bob", "alice");
        assert!(forward == reverse);

        let mut pairs = HashSet::new();
        pairs.insert(forward);
        assert!(pairs.contains(&reverse));
    }
}
