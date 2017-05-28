use std::fmt::Debug;
use heap::Heap;
use heap::binominal::BinominalHeap;

#[derive(Debug, PartialEq, Clone)]
enum ExplicitMin<T: Clone + Ord + Debug, H: Heap<T> + Clone> {
    Nil,
    Node(T, H),
}

impl<T: Clone + Ord + Debug, H: Heap<T> + Clone> Heap<T> for ExplicitMin<T, H> {
    fn empty() -> Self {
        ExplicitMin::Nil
    }

    fn insert(&self, x: T) -> Self {
        use self::ExplicitMin::*;
        match self {
            &Nil => Node(x, H::empty()),
            &Node(ref min, ref heap) => {
                if *min < x {
                    Node(min.clone(), heap.insert(x))
                } else {
                    Node(x, heap.insert(min.clone()))
                }
            }
        }
    }

    fn is_empty_heap(&self) -> bool {
        match self {
            &ExplicitMin::Nil => true,
            &ExplicitMin::Node(_, ref heap) => heap.is_empty_heap(),
        }
    }

    fn merge(&self, other: &Self) -> Self {
        use self::ExplicitMin::*;
        match (self, other) {
            (&Nil, _) => other.clone(),
            (_, &Nil) => self.clone(),
            (&Node(ref s_min, ref s_heap), &Node(ref o_min, ref o_heap)) => {
                if s_min < o_min {
                    Node(s_min.clone(), s_heap.merge(o_heap).insert(o_min.clone()))
                } else {
                    Node(o_min.clone(), o_heap.merge(s_heap).insert(s_min.clone()))
                }
            }
        }
    }

    fn find_min(&self) -> Option<T> {
        match self {
            &ExplicitMin::Nil => None,
            &ExplicitMin::Node(ref min, _) => Some(min.clone()),
        }
    }

    fn delete_min(&self) -> Self {
        use self::ExplicitMin::*;
        match self {
            &Nil => self.clone(),
            &Node(_, ref heap) => {
                match heap.find_min() {
                    None => self.clone(),
                    Some(min) => Node(min, heap.clone()),
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let actual_1 = ExplicitMin::empty().insert(3).insert(5).insert(1);
        let actual_2 = ExplicitMin::empty().insert(4).insert(8).insert(6);
        let actual: ExplicitMin<_, BinominalHeap<_>> = actual_1.merge(&actual_2);
        assert!(actual.find_min() == Some(1));
    }

    #[test]
    fn test_delete_min() {
        let actual: ExplicitMin<_, BinominalHeap<_>> = ExplicitMin::empty()
            .insert(1)
            .insert(5)
            .insert(3)
            .delete_min();
        assert!(actual.find_min() == Some(3));
    }

    #[test]
    fn test_find_min() {
        let actual: ExplicitMin<_, BinominalHeap<_>> =
            ExplicitMin::empty().insert(1).insert(5).insert(3);
        assert!(actual.find_min() == Some(1));
    }
}
