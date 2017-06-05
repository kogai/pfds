use std::fmt::Debug;
use list::{List, is_match_with_vec};
use list::linked_list::LinkedList;

use self::LinkedList::*;

#[derive(Debug, Clone, PartialEq)]
struct Deque<T: Clone + PartialOrd + PartialEq + Debug>(LinkedList<T>, LinkedList<T>);

impl<T> List<T> for Deque<T>
    where T: Clone + PartialEq + PartialOrd + Debug
{
    fn empty() -> Self {
        Deque(LinkedList::empty(), LinkedList::empty())
    }

    fn is_empty(&self) -> bool {
        match self {
            &Deque(Nil, _) => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        match self {
            &Deque(Nil, _) => Deque(Cons(x, box Nil), Nil),
            &Deque(ref fore, ref rear) => Deque(fore.cons(x), rear.clone()),
        }
    }

    fn head(&self) -> T {
        match self {
            &Deque(Nil, _) => unreachable!(),
            &Deque(ref fore, _) => fore.head(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &Deque(Nil, ref rear) |
            &Deque(Cons(_, box Nil), ref rear) => Deque(rear.reverse(), Nil),
            &Deque(Cons(_, box ref f_tail), ref rear) => Deque(f_tail.clone(), rear.clone()),
        }
    }

    fn concat(&self, _: Self) -> Self {
        unimplemented!();
    }
    fn update(&self, _: i32, _: T) -> Self {
        unimplemented!();
    }
}
impl<T> Deque<T>
    where T: Clone + PartialEq + PartialOrd + Debug
{
    fn snoc(&self, x: T) -> Self {
        match self {
            &Deque(Nil, _) => Deque(Cons(x, box Nil), Nil),
            &Deque(ref fore, ref rear) => Deque(fore.clone(), Cons(x, box rear.clone())),
        }
    }

    fn last(&self) -> T {
        match self {
            &Deque(Nil, _) => unreachable!(),
            &Deque(ref fore, _) => fore.head(),
        }
    }

    fn init(&self) -> Self {
        match self {
            &Deque(Nil, ref remain) |
            &Deque(ref remain, Nil) => unimplemented!(),
            &Deque(Cons(_, box Nil), ref rear) => Deque(rear.reverse(), Nil),
            &Deque(Cons(_, box ref f_tail), ref rear) => Deque(f_tail.clone(), rear.clone()),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let actual = Deque::empty().snoc(1).snoc(2).snoc(3);
        assert!(is_match_with_vec(actual.init(), vec![1, 2]));
        assert!(is_match_with_vec(actual.init().init(), vec![1]));
    }

    #[test]
    fn test_tail() {
        let actual = Deque::empty().snoc(1).snoc(2).snoc(3);
        assert!(is_match_with_vec(actual.tail(), vec![2, 3]));
        assert!(is_match_with_vec(actual.tail().tail(), vec![3]));
    }

    #[test]
    fn test_snoc() {
        let actual = Deque::empty().snoc(1).snoc(2).snoc(3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    #[test]
    fn test_cons() {
        let actual = Deque::empty().cons(3).cons(2).cons(1);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }
}

