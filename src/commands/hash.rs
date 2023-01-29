use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn _hash<T : Hash>(value : &T) -> u64 {
    let mut hasher = DefaultHasher::new();

    value.hash(&mut hasher);
    hasher.finish()
}