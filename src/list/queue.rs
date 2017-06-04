use std::fmt::Debug;
use list::{List, is_match_with_vec};
use list::linked_list::LinkedList;

use self::LinkedList::*;

#[derive(Debug, Clone, PartialEq)]
struct BatchedQueue<T: Clone + PartialOrd + PartialEq + Debug>(LinkedList<T>, LinkedList<T>);

impl<T> List<T> for BatchedQueue<T>
    where T: Clone + PartialEq + PartialOrd + Debug
{
    fn empty() -> Self {
        BatchedQueue(LinkedList::empty(), LinkedList::empty())
    }

    fn is_empty(&self) -> bool {
        match self {
            &BatchedQueue(Nil, _) => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        match self {
            &BatchedQueue(Nil, _) => BatchedQueue(Cons(x, box Nil), Nil),
            &BatchedQueue(ref fore, ref rear) => {
                BatchedQueue(fore.clone(), Cons(x, box rear.clone()))
            }
        }
    }

    fn head(&self) -> T {
        match self {
            &BatchedQueue(Nil, _) => unreachable!(),
            &BatchedQueue(ref fore, _) => fore.head(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &BatchedQueue(Nil, ref rear) |
            &BatchedQueue(Cons(_, box Nil), ref rear) => BatchedQueue(rear.reverse(), Nil),
            &BatchedQueue(Cons(_, box ref f_tail), ref rear) => {
                BatchedQueue(f_tail.clone(), rear.clone())
            }
        }
    }

    fn concat(&self, _: Self) -> Self {
        unimplemented!();
    }
    fn update(&self, _: i32, _: T) -> Self {
        unimplemented!();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_cons() {
        let actual = BatchedQueue::empty().cons(1).cons(2).cons(3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }
}

