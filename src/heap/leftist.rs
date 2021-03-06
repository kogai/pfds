use std::fmt::Debug;
use heap::Heap;

#[derive(Debug, PartialEq, Clone)]
pub enum LeftistHeap<T: Clone + Ord + Debug> {
    Leaf,
    Node(i32, T, Box<LeftistHeap<T>>, Box<LeftistHeap<T>>),
}

use self::LeftistHeap::*;

impl<T: Clone + Ord + Debug> LeftistHeap<T> {
    fn from_list_impl(xs: Vec<LeftistHeap<T>>) -> Self {

        match (&xs).len() {
            0 => Leaf,
            1 => xs.first().unwrap().clone(),
            _ => {
                LeftistHeap::from_list_impl(xs.chunks(2)
                                                .map(|x| {
                                                         x.iter().fold(Leaf, |acc, n| acc.merge(n))
                                                     })
                                                .collect::<Vec<_>>())
            }
        }
    }

    fn from_list(xs: Vec<T>) -> Self {
        LeftistHeap::from_list_impl(xs.into_iter()
                                        .map(|x| Node(1, x, box Leaf, box Leaf))
                                        .collect::<Vec<_>>())
    }

    fn rank(&self) -> i32 {
        match self {
            &Node(rank, _, _, _) => rank,
            &Leaf => 0,
        }
    }

    fn make_tree(&self, other: &Self, root: T) -> Self {
        let self_rank = self.rank();
        let other_rank = other.rank();
        let rank = if self_rank >= other_rank {
            self_rank
        } else {
            other_rank
        } + 1;

        // 高ランクの木を左に生やす
        if self_rank >= other_rank {
            Node(rank, root, box self.clone(), box other.clone())
        } else {
            Node(rank, root, box other.clone(), box self.clone())
        }
    }

    fn insert_with_wb(&self, x: T) -> Self {
        self.merge_with_wb(&Node(1, x, box Leaf, box Leaf))
    }

    fn merge_with_wb(&self, other: &Self) -> Self {
        match (self, other) {
            (&Leaf, &Node(_, _, _, _)) => other.clone(),
            (&Node(_, _, _, _), &Leaf) |
            (&Leaf, &Leaf) => self.clone(),
            (&Node(ref s_rank, ref s_element, ref s_left, ref s_right),
             &Node(ref o_rank, ref o_element, ref o_left, ref o_right)) => {
                let rank = s_rank + o_rank + 1;
                let (element, left, right) = if s_element <= o_element {
                    if s_rank > o_rank {
                        (s_element, s_left.merge(s_right), other.clone())
                    } else {
                        (s_element, other.clone(), s_left.merge(s_right))
                    }
                } else {
                    if s_rank > o_rank {
                        (o_element, self.clone(), o_left.merge(o_right))
                    } else {
                        (o_element, o_left.merge(o_right), self.clone())
                    }
                };
                Node(rank, element.clone(), box left, box right)
            }
        }
    }
}

impl<T: Clone + Ord + Debug> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        LeftistHeap::Leaf
    }

    fn is_empty_heap(&self) -> bool {
        match self {
            &Leaf => true,
            _ => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        match self {
            &Node(_, ref root, ref left, ref right) => {
                if &x <= root {
                    right.make_tree(&left.insert(root.clone()), x.clone())
                } else {
                    left.make_tree(&right.insert(x), root.clone())
                }
            }
            &Leaf => Node(1, x, box Leaf, box Leaf),
        }
    }

    fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            (&Leaf, &Node(_, _, _, _)) => other.clone(),
            (&Node(_, _, _, _), &Leaf) |
            (&Leaf, &Leaf) => self.clone(),
            (&Node(_, ref s_element, ref s_left, ref s_right),
             &Node(_, ref o_element, ref o_left, ref o_right)) => {
                if s_element <= o_element {
                    s_left.make_tree(&s_right.merge(other), s_element.clone())
                } else {
                    o_left.make_tree(&o_right.merge(self), o_element.clone())
                }
            }
        }
    }

    fn find_min(&self) -> Option<T> {
        match self {
            &Leaf => None,
            &Node(_, ref x, _, _) => Some(x.clone()),
        }
    }

    fn delete_min(&self) -> Self {
        match self {
            &Leaf => self.clone(),
            &Node(_, _, ref left, ref right) => left.merge(right),
        }
    }
}

mod tests {
    use super::*;

    fn create_node<T: Clone + Ord + Debug>(x: T) -> LeftistHeap<T> {
        LeftistHeap::Node(1, x, box LeftistHeap::Leaf, box LeftistHeap::Leaf)
    }

    fn is_leftist_inner<T: Clone + Ord + Debug>(left: &LeftistHeap<T>,
                                                right: &LeftistHeap<T>)
                                                -> bool {
        match (left, right) {
            (&Node(_, _, box ref left, box ref right), &Leaf) => is_leftist_inner(left, right),
            (&Leaf, &Node(_, _, _, _)) => false,
            (&Leaf, &Leaf) => true,
            (&Node(l_rank, _, box ref l_left, box ref l_right),
             &Node(r_rank, _, box ref r_left, box ref r_right)) => {
                l_rank >= r_rank && is_leftist_inner(l_left, l_right) &&
                is_leftist_inner(r_left, r_right)
            }
        }
    }

    fn is_leftist<T: Clone + Ord + Debug>(x: &LeftistHeap<T>) -> bool {
        if let &Node(_, _, box ref left, box ref right) = x {
            is_leftist_inner(left, right)
        } else {
            false
        }
    }

    fn is_ordered<T: Clone + Ord + Debug>(x: &LeftistHeap<T>, min: &T) -> bool {
        match x {
            &Leaf => true,
            &Node(_, ref element, box ref left, box ref right) => {
                (min <= element) && is_ordered(left, element) && is_ordered(right, element)
            }
        }
    }

    #[test]
    fn test_weight_biased_leftist() {
        let actual = LeftistHeap::Leaf
            .insert_with_wb(5)
            .insert_with_wb(6)
            .insert_with_wb(4)
            .insert_with_wb(1)
            .insert_with_wb(3)
            .insert_with_wb(3)
            .insert_with_wb(2);

        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_find_min() {
        let actual = create_node(3).insert(2).insert(1);
        assert!(actual.find_min() == Some(1));
    }

    #[test]
    fn test_delete_min() {
        let actual = LeftistHeap::from_list(vec![1, 3, 2, 4, 10, 2, 4]);
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_from_list() {
        let actual = LeftistHeap::from_list(vec![1, 3, 2, 4]);
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_insert() {
        let actual = create_node(10).insert(20).insert(30).insert(40);
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_insert_large() {
        let actual = create_node(40).insert(30).insert(20).insert(10);
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_merge() {
        let actual = create_node(10).merge(&create_node(20));
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_merge_nest() {
        let actual = create_node(10)
            .merge(&create_node(20))
            .merge(&create_node(30));
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }

    #[test]
    fn test_make_tree() {
        let actual = create_node(10).make_tree(&create_node(20), 5);
        assert!(is_ordered(&actual, &actual.find_min().unwrap()));
        assert!(is_leftist(&actual));
    }
}

