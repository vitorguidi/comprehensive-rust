use std::collections::HashMap;
use std::hash::Hash;

pub struct HashMapCounter<T: Hash + Eq> {
    values: HashMap<T, u64>
}

pub trait Counter<T> {
    fn increment(self: &mut Self, key: T);
    fn count(self: &Self, key: T) -> u64;
}

impl<T: Hash + Eq> HashMapCounter<T> {
    pub fn new() -> HashMapCounter<T> {
        HashMapCounter{values: HashMap::new()}
    }
}

impl<T> Counter<T> for HashMapCounter<T> where T: Hash + Eq {
    fn increment(self: &mut Self, key: T) {
        *self.values.entry(key).or_default() += 1;
    }

    fn count(self: &Self, key: T) -> u64 {
        self.values.get(&key).copied().unwrap_or_default()
    }
}