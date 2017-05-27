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

type BinominalHeap<T: Clone + Ord + Debug> = Stack<BinominalTree<T>>;

impl<T: Clone + Ord + Debug> BinominalHeap<T> {
    fn with_binominal_tree(x: T) -> Self {
        Stack::new(BinominalTree::new(x))
    }

    fn insert_tree(&self, x: BinominalTree<T>) -> Self {
        match self {
            &Stack::Nil => self.cons(x),
            &Stack::Node(ref head, box ref tail) => {
                if x.rank < head.rank {
                    self.cons(x)
                } else {
                    tail.clone().insert_tree(x.link(head))
                }
            }
        }
    }

    fn remove_min_tree(&self) -> (BinominalTree<T>, Self) {
        match self {
            &Stack::Nil => unreachable!(),
            &Stack::Node(ref head, box ref tail) => {
                if tail.is_empty() {
                    (head.clone(), self.clone())
                } else {
                    let (o_head, o_tail) = tail.remove_min_tree();
                    if head.clone().element <= o_head.element {
                        (head.clone(), tail.clone())
                    } else {
                        (o_head.clone(), o_tail.cons(head.clone()))
                    }
                }
            }
        }
    }

    fn find_min_impl(&self, min: &T) -> T {
        match self {
            &Stack::Nil => min.clone(),
            &Stack::Node(ref head, box ref tail) => {
                if &head.element < min {
                   tail.find_min_impl(&head.element) 
                } else {
                   tail.find_min_impl(min) 
                }
            } 
        }
    }
}

impl<T: Clone + Ord + Debug> Heap<T> for BinominalHeap<T> {
    fn is_empty_heap(&self) -> bool {
        self.is_empty()
    }

    fn insert(&self, x: T) -> Self {
        self.insert_tree(BinominalTree::new(x))
    }

    fn merge(&self, other: &Self) -> Self {
        match (self.clone(), other.clone()) {
            (_, Stack::Nil) => self.clone(),
            (Stack::Nil, _) => other.clone(),
            (Stack::Node(s, box s_tail), Stack::Node(o, box o_tail)) => {
                match s.rank {
                    _ if s.rank < o.rank => s_tail.merge(other).cons(s),
                    _ if s.rank > o.rank => o_tail.merge(self).cons(o),
                    _ => s_tail.merge(&o_tail).insert_tree(s.link(&o)),
                }
            }
        }
    }

    fn find_min(&self) -> Option<T> {
        match self {
            &Stack::Nil => None,
            &Stack::Node(ref head, box ref tail) => Some(tail.find_min_impl(&head.element)),
        }
    }

    fn delete_min(&self) -> Self {
        match self {
            &Stack::Nil => self.clone(),
            _ => {
                let (head, tail) = self.remove_min_tree();
                head.children.reverse().merge(&tail)
            }
        }
    }
}

mod tests {
    use super::*;

    fn is_ordered_tree<T: Clone + Ord + Debug>(x: &BinominalTree<T>, min: &T) -> bool {
        match x.children.is_empty() {
            true => true,
            false => &x.element > min && x.children.all(&|c| is_ordered_tree(c, &x.element)),
        }
    }

    fn is_ordered_heap<T: Clone + Ord + Debug>(heap: &BinominalHeap<T>, min: &T) -> bool {
        heap.all(&|x| is_ordered_tree(x, min))
    }

    fn size_from_element<T: Clone + Ord + Debug>(x: &BinominalTree<T>) -> i32 {
        match x.children.is_empty() {
            true => 1,
            false => x.children.foldl(0, &|acc, x| acc + size_from_element(x)) + 1,
        }
    }

    // ランクrの二項木は2のr乗のノードを含む
    fn size_from_rank(rank: i32) -> i32 {
        (2 as i32).pow(rank as u32)
    }

    fn size_from_elements<T: Clone + Ord + Debug>(x: &BinominalHeap<T>) -> i32 {
        x.foldl(0, &|acc, x| acc + size_from_element(x))
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
            .fold((x.clone(), true), |(list, is_present_correct), rank| {
                (list.tail(),
                 is_present_correct && list.head().rank == rank &&
                 is_correspond_to_binary_representation(&list.tail()))
            })
            .1;
        result
    }

    #[test]
    fn test_delete_min() {
        let actual = BinominalHeap::with_binominal_tree(5)
            .insert(2)
            .insert(4)
            .insert(7)
            .insert(1)
            .delete_min();

        assert!(is_ordered_heap(&actual, &0));
        assert!(is_correspond_to_binary_representation(&actual));
        assert!(size_from_elements(&actual) == 4);
    }

    #[test]
    fn test_find_min() {
        let actual = BinominalHeap::with_binominal_tree(5)
            .insert(2)
            .insert(4)
            .insert(4)
            .insert(1)
            .insert(3)
            .find_min();

        assert!(actual == Some(1));
    }

    #[test]
    fn test_merge() {
        let actual_1 = BinominalHeap::with_binominal_tree(1)
            .insert(2)
            .insert(3)
            .insert(4)
            .insert(5);
        let actual_2 = BinominalHeap::with_binominal_tree(5)
            .insert(4)
            .insert(3)
            .insert(2)
            .insert(1);
        let actual = actual_1.merge(&actual_2);

        assert!(is_ordered_heap(&actual, &0));
        assert!(is_correspond_to_binary_representation(&actual));
    }

    #[test]
    fn test_insert() {
        let actual = BinominalHeap::with_binominal_tree(1)
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

