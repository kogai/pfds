use std::fmt::Debug;
use heap::Heap;

#[derive(Debug, PartialEq, Clone)]
enum BinominalHeap<T: Clone + Ord + Debug> {
    Leaf,
    Node {
        rank: i32,
        element: T,
        left: Box<BinominalHeap<T>>,
        right: Box<BinominalHeap<T>>,
    },
}

