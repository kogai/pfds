use std::fmt::Debug;
use heap::Heap;

#[derive(Debug)]
enum SplayHeap<T>
    where T: Clone + Ord + PartialEq + Debug
{
    Empty,
    Tree {
        element: T,
        left: Box<SplayHeap<T>>,
        right: Box<SplayHeap<T>>,
    },
}

use self::SplayHeap::*;


impl<T> SplayHeap<T>
    where T: Clone + Ord + PartialEq + Debug
{
    fn bigger(&self, pivot: &T) -> Self {
        unimplemented!();
    }

    fn smaller(&self, pivot: &T) -> Self {
        unimplemented!();
    }

    fn from_vec(xs: Vec<T>) -> Self {
        xs.iter()
            .fold(SplayHeap::empty(), |acc, x| acc.insert(x.clone()))
    }
}

impl<T> Heap<T> for SplayHeap<T>
    where T: Clone + Ord + PartialEq + Debug
{
    fn empty() -> Self {
        Empty
    }
    fn is_empty_heap(&self) -> bool {
        match self {
            &Empty => true,
            _ => false,
        }
    }
    fn insert(&self, x: T) -> Self {
        Tree {
            left: box self.smaller(&x),
            right: box self.bigger(&x),
            element: x,
        }
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

    #[test]
    fn test_insert() {
        let actual = SplayHeap::from_vec(vec![3, 2, 5, 1, 6, 4]);
        assert!(is_left_node_small(&actual));
        assert!(is_right_node_big(&actual));
    }

    fn is_left_node_small_impl<T>(tree: &SplayHeap<T>, pivot: &T) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree {
                ref element,
                ref left,
                ..
            } => pivot >= element && is_left_node_small_impl(left, element),
        }
    }

    fn is_left_node_small<T>(tree: &SplayHeap<T>) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree {
                ref element,
                ref left,
                ..
            } => is_left_node_small_impl(left, element),
        }
    }

    fn is_right_node_small_impl<T>(tree: &SplayHeap<T>, pivot: &T) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree {
                ref element,
                ref right,
                ..
            } => pivot < element && is_right_node_small_impl(right, element),
        }
    }

    fn is_right_node_big<T>(tree: &SplayHeap<T>) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree {
                ref element,
                ref right,
                ..
            } => is_right_node_small_impl(right, element),
        }
    }
}

