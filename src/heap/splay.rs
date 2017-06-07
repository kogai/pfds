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

        println!("{:?}", actual);

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

