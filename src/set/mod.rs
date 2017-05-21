use std::fmt::Debug;

mod tree;
mod map;

trait Set<T: Ord + Clone + Debug> {
    fn empty() -> Self;
    fn member(&self, x: &T) -> bool;
    fn insert(&self, x: T) -> Self;
}

trait Sequence {
    fn to_successor_with(&self, skip: i32) -> Self;
    fn to_predecessor_with(&self, skip: i32) -> Self;
}

trait FiniteMap<K, V> {
    fn empty() -> Self;
    fn bind(&self, key: K, value: V) -> Self;
    fn lookup(&self, key: K) -> Option<V>;
}
