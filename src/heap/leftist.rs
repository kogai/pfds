use std::fmt::Debug;
use heap::Heap;

#[derive(Debug, PartialEq, Clone)]
enum LeftistHeap<T: Clone + Ord + Debug> {
    Leaf,
    Node(i32, T, Box<LeftistHeap<T>>, Box<LeftistHeap<T>>),
}

impl<T: Clone + Ord + Debug> LeftistHeap<T> {
    fn rank(&self) -> i32 {
        match self {
            &LeftistHeap::Node(rank, _, _, _) => rank,
            &LeftistHeap::Leaf => 0,
        }
    }

    fn make_tree(&self, other: &Self, root: T) -> Self {
        use self::LeftistHeap::*;
        // x < self.element && x < other.element が前提
        // ランクの低い方の部分木を元に根のランクを算出し、右ノードに生やす

        let self_rank = self.rank();
        let other_rank = other.rank();

        if self_rank < other_rank {
            Node(self_rank + 1, root, box other.clone(), box self.clone())
        } else {
            Node(other_rank + 1, root, box self.clone(), box other.clone())
        }
    }

    fn from_list_impl(xs: Vec<LeftistHeap<T>>) -> Self {
        use self::LeftistHeap::*;

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
        use self::LeftistHeap::*;

        LeftistHeap::from_list_impl(xs.into_iter()
                                        .map(|x| Node(1, x, box Leaf, box Leaf))
                                        .collect::<Vec<_>>())
    }
}

impl<T: Clone + Ord + Debug> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        LeftistHeap::Leaf
    }

    fn is_empty(&self) -> bool {
        match self {
            &LeftistHeap::Leaf => true,
            _ => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        use self::LeftistHeap::*;
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
        use self::LeftistHeap::*;
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
        use self::LeftistHeap::*;
        match self {
            &Leaf => None,
            &Node(_, ref x, _, _) => Some(x.clone()),
        }
    }

    fn delete_min(&self) -> Self {
        use self::LeftistHeap::*;
        match self {
            &Leaf => self.clone(),
            &Node(_, _, ref left, ref right) => left.merge(right),
        }
    }
}

mod tests {
    use super::*;

    fn create_node<T: Clone + Ord + Debug>(x: T) -> LeftistHeap<T> {
        LeftistHeap::Node(1, x, box LeftistHeap::empty(), box LeftistHeap::empty())
    }

    fn assert_leftist<T: Clone + Ord + Debug>(left: LeftistHeap<T>, right: LeftistHeap<T>) {
        use self::LeftistHeap::*;
        match (left, right) {
            (_, Leaf) => (),
            (Leaf, Node(_, _, _, _)) => assert!(false),
            (Node(l_rank, _, box l_left, box l_right),
             Node(r_rank, _, box r_left, box r_right)) => {
                assert!(l_rank >= r_rank);
                assert_leftist(l_left, l_right);
                assert_leftist(r_left, r_right);
            }
        }
    }

    #[test]
    fn test_find_min() {
        let actual = create_node(3).insert(2).insert(1);
        assert!(actual.find_min() == Some(1));
    }

    #[test]
    fn test_delete_min() {
        use self::LeftistHeap::*;
        let actual = LeftistHeap::from_list(vec![1, 3, 2, 4, 10, 2, 4]);
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_from_list() {
        use self::LeftistHeap::*;
        let actual = LeftistHeap::from_list(vec![1, 3, 2, 4]);
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_insert() {
        use self::LeftistHeap::*;
        let actual = create_node(10).insert(20).insert(30).insert(40);
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_insert_large() {
        use self::LeftistHeap::*;
        let actual = create_node(40).insert(30).insert(20).insert(10);
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_merge() {
        use self::LeftistHeap::*;
        let actual = create_node(10).merge(&create_node(20));
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_merge_nest() {
        use self::LeftistHeap::*;
        let actual = create_node(10)
            .merge(&create_node(20))
            .merge(&create_node(30));
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }

    #[test]
    fn test_make_tree() {
        use self::LeftistHeap::*;
        let actual = create_node(10).make_tree(&create_node(20), 5);
        if let Node(_, _, box left, box right) = actual {
            assert_leftist(left, right);
        }
    }
}

