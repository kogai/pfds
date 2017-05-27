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
        assert!(self.rank == that.rank);

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

    fn is_ordered<T: Clone + Ord + Debug>(x: &BinominalHeap<T>, min: &T) -> bool {
        match x.children.is_empty() {
            true => true,
            false => &x.element > min && x.children.all(&|c| is_ordered(c, &x.element)),
        }
    }

    // ランクrの二項木は2のr乗のノードを含む
    fn count_node<T: Clone + Ord + Debug>(x: &BinominalHeap<T>, rank: i32) -> i32 {
        match x.children.is_empty() {
            true => 1,
            false => x.children.foldl(0, &|acc, x| acc + count_node(x, x.rank)) + 1,
        }
    }

    fn derive_count_node_from(rank: i32) -> i32 {
        (2 as i32).pow(rank as u32)
    }

    #[test]
    fn test_link() {
        let actual_1 = BinominalHeap::new(1).link(&BinominalHeap::new(2));
        let actual_2 = BinominalHeap::new(3).link(&BinominalHeap::new(4));
        let actual = actual_1.link(&actual_2);
        assert!(is_ordered(&actual, &0));
        assert!(count_node(&actual, actual.rank) == derive_count_node_from(actual.rank));
    }
}

