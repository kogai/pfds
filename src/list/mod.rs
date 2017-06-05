use std::fmt::Debug;

pub mod stack;
pub mod stream;
pub mod linked_list;
pub mod queue;
pub mod deque;

pub trait List<T: Clone>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;
    fn concat(&self, ys: Self) -> Self;
    fn update(&self, index: i32, x: T) -> Self;
}

pub fn is_match_with_vec<T, L>(xs: L, ys: Vec<T>) -> bool
    where T: Debug + PartialEq + PartialOrd + Clone,
            L: List<T>
{
    if ys.is_empty() {
        return xs.is_empty()
    }
    ys.iter()
        .fold((xs, true), |(xs, prev), y| {
            let head = xs.head();
            let tail = xs.tail();
            (tail, prev && &head == y)
        })
        .1
}

