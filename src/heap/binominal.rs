use std::fmt::Debug;
use heap::Heap;
use list::{Stack, List};

#[derive(Debug, PartialEq, Clone)]
struct BinominalHeap<T: Clone + Ord + Debug> {
    rank: i32,
    element: T,
    children: Box<Stack<BinominalHeap<T>>>,
}

impl<T: Clone + Ord + Debug> BinominalHeap<T> {
    fn new(element: T) -> Self {
        BinominalHeap {
            rank: 0,
            element,
            children: box Stack::Nil,
        }
    }

    fn link(&self, that: &Self) -> Self {
        if self.element <= that.element {
            BinominalHeap {
                rank: self.rank + 1,
                element: self.element.clone(),
                children: box self.children.cons(that.clone()),
            }
        } else {
            BinominalHeap {
                rank: self.rank + 1,
                element: that.element.clone(),
                children: box that.children.cons(self.clone()),
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_link() {
        let actual = BinominalHeap::new(20).link(&BinominalHeap::new(10));
        let expect = BinominalHeap {
            rank: 1,
            element: 10,
            children: box Stack::Node(BinominalHeap::new(20), box Stack::Nil),
        };

        assert!(actual == expect);
    }
}

