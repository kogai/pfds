use std::fmt::Debug;
use heap::Heap;

#[derive(Debug, Clone)]
enum SplayHeap<T>
    where T: Clone + Ord + PartialEq + Debug
{
    Empty,
    Tree(Box<SplayHeap<T>>, T, Box<SplayHeap<T>>),
}

use self::SplayHeap::*;


impl<T> SplayHeap<T>
    where T: Clone + Ord + PartialEq + Debug
{
    fn bigger(&self, pivot: &T) -> Self {
        match self {
            &Empty => Empty,
            &Tree(box ref left, ref x, box ref right) => {
                if x <= pivot {
                    right.bigger(pivot)
                } else {
                    match left {
                        &Empty => Tree(box Empty, x.clone(), box right.clone()),
                        &Tree(box ref left2, ref y, box ref right2) => {
                            if y <= pivot {
                                Tree(box right2.bigger(pivot), x.clone(), box right.clone())
                            } else {
                                Tree(box left2.bigger(pivot),
                                     y.clone(),
                                     box Tree(box right2.clone(), x.clone(), box right.clone()))
                            }
                        }
                    }
                }
            }
        }
    }

    fn smaller(&self, pivot: &T) -> Self {
        match self {
            &Empty => Empty,
            &Tree(box ref left, ref x, box ref right) => {
                if x > pivot {
                    left.smaller(pivot)
                } else {
                    match right {
                        &Empty => Tree(box left.clone(), x.clone(), box Empty),
                        &Tree(box ref left2, ref y, box ref right2) => {
                            if y > pivot {
                                Tree(box left.clone(), x.clone(), box right2.smaller(pivot))
                            } else {
                                Tree(box Tree(box left.clone(), x.clone(), box left2.clone()),
                                     y.clone(),
                                     box right2.smaller(pivot))
                            }
                        }
                    }
                }
            }
        }
    }

    fn from_vec(xs: Vec<T>) -> Self {
        xs.into_iter()
            .fold(SplayHeap::empty(), |acc, x| acc.insert(x))
    }

    fn len(&self) -> i32 {
        match self {
            &Empty => 0,
            &Tree(box ref left, _, box ref right) => 1 + left.len() + right.len(),
        }
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
        Tree(box self.smaller(&x), x.clone(), box self.bigger(&x))
    }
    fn merge(&self, other: &Self) -> Self {
        match self {
            &Empty => other.clone(),
            &Tree(box ref left, ref x, box ref right) => {
                let small = other.smaller(x);
                let big = other.bigger(x);
                Tree(box left.merge(&small), x.clone(), box right.merge(&big))
            }
        }
    }

    fn find_min(&self) -> Option<T> {
        match self {
            &Empty => None,
            &Tree(box Empty, ref x, _) => Some(x.clone()),
            &Tree(box ref left, _, _) => left.find_min(),
        }
    }

    fn delete_min(&self) -> Self {
        match self {
            &Empty => Empty,
            &Tree(box Empty, _, box ref right) => right.clone(),
            &Tree(box Tree(ref left, ref x, ref right), ref y, ref right2) => {
                Tree(box left.delete_min(),
                     x.clone(),
                     box Tree(right.clone(), y.clone(), right2.clone()))
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let actual_1 = SplayHeap::from_vec(vec![3, 2, 5]);
        let actual_2 = SplayHeap::from_vec(vec![1, 6, 4]);
        let actual = actual_1.merge(&actual_2);
        assert!(actual.len() == 6);
        assert!(is_left_node_small(&actual));
        assert!(is_right_node_big(&actual));
    }

    #[test]
    fn test_delete_min() {
        let actual = SplayHeap::from_vec(vec![3, 2, 5, 1, 6, 4]).delete_min();
        assert!(actual.len() == 5);
        assert!(actual.find_min() == Some(2));
        assert!(is_left_node_small(&actual));
        assert!(is_right_node_big(&actual));
    }

    #[test]
    fn test_find_min() {
        let actual = SplayHeap::from_vec(vec![3, 2, 5, 1, 6, 4]).find_min();
        assert!(actual == Some(1));
    }

    #[test]
    fn test_insert() {
        let actual = SplayHeap::from_vec(vec![3, 2, 5, 1, 6, 4]);
        assert!(actual.len() == 6);
        assert!(is_left_node_small(&actual));
        assert!(is_right_node_big(&actual));
    }

    fn is_left_node_small_impl<T>(tree: &SplayHeap<T>, pivot: &T) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree(ref left, ref element, _) => {
                pivot >= element && is_left_node_small_impl(left, element)
            }
        }
    }

    fn is_left_node_small<T>(tree: &SplayHeap<T>) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree(ref left, ref element, _) => is_left_node_small_impl(left, element),
        }
    }

    fn is_right_node_small_impl<T>(tree: &SplayHeap<T>, pivot: &T) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree(_, ref element, ref right) => {
                pivot < element && is_right_node_small_impl(right, element)
            }
        }
    }

    fn is_right_node_big<T>(tree: &SplayHeap<T>) -> bool
        where T: Clone + Ord + PartialEq + Debug
    {
        match tree {
            &Empty => true,
            &Tree(_, ref element, ref right) => is_right_node_small_impl(right, element),
        }
    }
}

