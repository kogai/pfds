use std::fmt::Debug;
use list::List;
use list::linked_list::LinkedList;

use self::LinkedList::*;

#[derive(Debug, Clone, PartialEq)]
struct BatchedQueue<T: Clone + PartialOrd + PartialEq + Debug>(LinkedList<T>, LinkedList<T>);

impl<T> BatchedQueue<T>
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
}

mod tests {
    use super::*;

    #[test]
    fn test_cons() {
        let actual = BatchedQueue::empty().cons(1).cons(2).cons(3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    fn is_match_with_vec<T>(xs: BatchedQueue<T>, ys: Vec<T>) -> bool
        where T: Debug + PartialEq + PartialOrd + PartialOrd + Clone
    {
        ys.iter()
            .fold((xs, true), |(xs, prev), y| {
                let head = xs.head();
                let tail = xs.tail();
                (tail, prev && &head == y)
            })
            .1
    }
}

