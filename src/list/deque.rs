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
            &Deque(Nil, Nil) => Deque(Nil, Cons(x, box Nil)),
            &Deque(Nil, ref rear) => {
                let (f, r) = rear.reverse().split();
                Deque(f.cons(x), r)
            }
            &Deque(ref fore, Nil) => {
                let (f, r) = fore.split();
                Deque(f.cons(x), r)
            }
            &Deque(ref fore, ref rear) => Deque(fore.cons(x), rear.clone()),
        }
    }

    fn head(&self) -> T {
        match self {
            &Deque(Nil, Nil) => unreachable!(),
            &Deque(Nil, ref rear) => rear.reverse().head(),
            &Deque(ref fore, _) => fore.head(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &Deque(Nil, Nil) => self.clone(),
            &Deque(Nil, ref rear) => {
                let (f, r) = rear.reverse().split();
                Deque(f.tail(), r.reverse())
            }
            &Deque(ref fore, Nil) => {
                let (f, r) = fore.split();
                Deque(f.tail(), r.reverse())
            }
            &Deque(Cons(_, box ref tail), ref rear) => Deque(tail.clone(), rear.clone()),
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
            &Deque(Nil, Nil) => Deque(Cons(x, box Nil), Nil),
            &Deque(Nil, ref rear) => {
                let (f, r) = rear.reverse().split();
                Deque(f, r.cons(x))
            }
            &Deque(ref fore, Nil) => {
                let (f, r) = fore.split();
                Deque(f, r.cons(x))
            }
            &Deque(ref fore, ref rear) => Deque(fore.clone(), rear.cons(x)),
        }
    }

    fn last(&self) -> T {
        match self {
            &Deque(Nil, Nil) => unreachable!(),
            &Deque(ref fore, Nil) => fore.reverse().head(),
            &Deque(_, ref rear) => rear.head(),
        }
    }

    fn init(&self) -> Self {
        match self {
            &Deque(Nil, Nil) => self.clone(),
            &Deque(Nil, ref rear) => {
                let (f, r) = rear.reverse().split();
                Deque(f, r.reverse().init())
            }
            &Deque(ref fore, Nil) => {
                let (f, r) = fore.split();
                Deque(f, r.reverse().init())
            }
            &Deque(ref fore, Cons(_, box ref tail)) => Deque(fore.clone(), tail.clone()),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_last() {
        let actual = Deque::empty().snoc(1).snoc(2).snoc(3);
        assert!(actual.last() == 3);
    }

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

