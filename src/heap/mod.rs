use std::fmt::Debug;

mod leftist;
mod binominal;
mod explicit_min;

trait Heap<T: Clone + Debug + Ord> {
    fn empty() -> Self;
    fn is_empty_heap(&self) -> bool;
    fn insert(&self, x: T) -> Self;
    fn merge(&self, other: &Self) -> Self;
    fn find_min(&self) -> Option<T>;
    fn delete_min(&self) -> Self;
}