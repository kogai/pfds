use std::fmt::Debug;
use heap::Heap;
use list::{Stack, List};

#[derive(Debug, PartialEq, Clone)]
struct BinominalTree<T: Clone + Ord + Debug> {
    rank: i32,
    element: T,
    children: Box<Stack<BinominalTree<T>>>,
}

impl<T: Clone + Ord + Debug> BinominalTree<T> {
    fn new(element: T) -> Self {
        BinominalTree {
            rank: 0,
            element,
            children: box Stack::Nil,
        }
    }

    fn link(&self, that: &Self) -> Self {
        assert!(self.rank == that.rank);

        if self.element <= that.element {
            BinominalTree {
                rank: self.rank + 1,
                element: self.element.clone(),
                children: box self.children.cons(that.clone()),
            }
        } else {
            BinominalTree {
                rank: self.rank + 1,
                element: that.element.clone(),
                children: box that.children.cons(self.clone()),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BinominalHeap<T: Clone + Ord + Debug>(Stack<BinominalTree<T>>);

impl<T: Clone + Ord + Debug> BinominalHeap<T> {
    fn new(x: T) -> Self {
        BinominalHeap(Stack::new(BinominalTree::new(x)))
    }

    fn insert_tree(&self, x: BinominalTree<T>) -> Self {
        match self.0 {
            Stack::Nil => BinominalHeap(self.0.cons(x)),
            Stack::Node(ref head, box ref tail) => {
                if x.rank < head.rank {
                    BinominalHeap(self.0.cons(x))
                } else {
                    BinominalHeap(tail.clone()).insert_tree(x.link(head))
                }
            }
        }
    }
}

impl<T: Clone + Ord + Debug> Heap<T> for BinominalHeap<T> {
    fn is_empty(&self) -> bool {
        unimplemented!();
    }

    fn insert(&self, x: T) -> Self {
        self.insert_tree(BinominalTree::new(x))
    }

    fn merge(&self, other: &Self) -> Self {
        unimplemented!();
    }
    fn find_min(&self) -> Option<T> {
        unimplemented!();
    }
    fn delete_min(&self) -> Self {
        unimplemented!();
    }
}

mod tests {
    use super::*;

    // TODO: 重複したTの場合
    fn is_ordered_tree<T: Clone + Ord + Debug>(x: &BinominalTree<T>, min: &T) -> bool {
        match x.children.is_empty() {
            true => true,
            false => &x.element > min && x.children.all(&|c| is_ordered_tree(c, &x.element)),
        }
    }

    fn is_ordered_heap<T: Clone + Ord + Debug>(heap: &BinominalHeap<T>, min: &T) -> bool {
        heap.0.all(&|x| is_ordered_tree(x, min))
    }

    // ランクrの二項木は2のr乗のノードを含む
    fn size_from_element<T: Clone + Ord + Debug>(x: &BinominalTree<T>) -> i32 {
        match Stack::is_empty(&x.children) {
            true => 1,
            false => x.children.foldl(0, &|acc, x| acc + size_from_element(x)) + 1,
        }
    }

    fn size_from_rank(rank: i32) -> i32 {
        (2 as i32).pow(rank as u32)
    }

    fn size_from_elements<T: Clone + Ord + Debug>(x: &BinominalHeap<T>) -> i32 {
        x.0.foldl(0, &|acc, x| acc + size_from_element(x))
    }

    // サイズ6の二項ヒープ -> 110(二進表記) -> ランク1とランク2の二項木の集合になっていること
    fn is_correspond_to_binary_representation<T: Clone + Ord + Debug>(x: &BinominalHeap<T>)
                                                                      -> bool {
        let result = format!("{:b}", size_from_elements(x))
            .chars()
            .rev()
            .enumerate()
            .filter(|&(_, b)| b != '0')
            .map(|(rank, _)| rank as i32)
            .fold((x.0.clone(), true), |(list, is_present_correct), rank| {
                (list.tail(), is_present_correct && list.head().rank == rank)
            })
            .1;
        result
    }

    #[test]
    fn test_insert() {
        let actual = BinominalHeap::new(1)
            .insert(4)
            .insert(2)
            .insert(6)
            .insert(7)
            .insert(3)
            .insert(5);

        assert!(is_ordered_heap(&actual, &0));
        assert!(is_correspond_to_binary_representation(&actual));
    }

    #[test]
    fn test_link() {
        let actual_1 = BinominalTree::new(1).link(&BinominalTree::new(2));
        let actual_2 = BinominalTree::new(3).link(&BinominalTree::new(4));
        let actual = actual_1.link(&actual_2);

        assert!(is_ordered_tree(&actual, &0));
        assert!(size_from_element(&actual) == size_from_rank(actual.rank));
    }
}

