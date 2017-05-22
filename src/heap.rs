use std::fmt::Debug;

trait Heap<T: Clone + Debug> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn insert(&self, x: T) -> Self;
    fn merge(&self, other: &Self) -> Self;
    fn find_min(&self) -> Option<T>;
    fn delete_min(&self) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
enum LeftistHeap<T: Clone + Ord + Debug> {
    Nil,
    Element(i32, T, Box<LeftistHeap<T>>, Box<LeftistHeap<T>>),
}

impl<T: Clone + Ord + Debug> LeftistHeap<T> {
    fn rank(&self) -> i32 {
        match self {
            &LeftistHeap::Element(rank, _, _, _) => rank,
            &LeftistHeap::Nil => 0,
        }
    }

    fn make_tree(&self, other: &Self, x: T) -> Self {
        use self::LeftistHeap::*;
        // x < self.element && x < other.element が前提
        // ランクの低い方の部分木を元に根のランクを算出し、右ノードに生やす

        let self_rank = self.rank();
        let other_rank = other.rank();

        if self_rank < other_rank {
            Element(self_rank + 1, x, box other.clone(), box self.clone())
        } else {
            Element(other_rank + 1, x, box self.clone(), box other.clone())
        }
    }
}

impl<T: Clone + Ord + Debug> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        LeftistHeap::Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            &LeftistHeap::Nil => true,
            _ => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        unimplemented!();
    }

    fn merge(&self, other: &Self) -> Self {
        use self::LeftistHeap::*;
        match (self, other) {
            (&Nil, &Element(_, _, _, _)) => other.clone(),
            (&Element(_, _, _, _), &Nil) |
            (&Nil, &Nil) => self.clone(),
            (&Element(_, ref s_element, ref s_left, ref s_right),
             &Element(_, ref o_element, ref o_left, ref o_right)) => {
                if s_element <= o_element {
                    s_left.make_tree(&s_right.merge(other), s_element.clone())
                } else {
                    o_left.make_tree(&o_right.merge(self), o_element.clone())
                }
            }
        }
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
    fn create_node<T: Clone + Ord + Debug>(x: T) -> LeftistHeap<T> {
        LeftistHeap::Element(1, x, box LeftistHeap::empty(), box LeftistHeap::empty())
    }

    #[test]
    fn test_merge() {
        use self::LeftistHeap::*;
        let actual = create_node(10).merge(&create_node(20));
        let expect = Element(1, 10, box create_node(20), box Nil);
        assert!(actual == expect);
    }

    #[test]
    fn test_merge_nest() {
        use self::LeftistHeap::*;
        let actual = create_node(10)
            .merge(&create_node(20))
            .merge(&create_node(30));
        let expect = Element(2, 10, box create_node(20), box create_node(30));
        assert!(actual == expect);
    }

    #[test]
    fn test_make_tree() {
        use self::LeftistHeap::*;
        let actual = create_node(10).make_tree(&create_node(20), 5);
        assert!(actual == Element(2, 5, box create_node(10), box create_node(20)));
    }
}

