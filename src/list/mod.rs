pub mod stack;

pub trait List<T: Clone>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;
    fn concat(&self, ys: Self) -> Self;
    fn update(&self, index: i32, x: T) -> Self;
}

