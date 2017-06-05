use std::fmt::Debug;
use list::{List, is_match_with_vec};

#[derive(Debug, Clone, PartialEq)]
pub enum LinkedList<T: Debug + PartialEq + PartialOrd + Clone> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

use self::LinkedList::*;

impl<T> List<T> for LinkedList<T>
    where T: Debug + PartialEq + PartialOrd + Clone
{
    fn empty() -> Self {
        Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            &Nil => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        match self {
            &Nil => Cons(x, box Nil),
            _ => Cons(x, box self.clone()),
        }
    }

    fn head(&self) -> T {
        match self {
            &Nil => unreachable!(),
            &Cons(ref head, _) => head.clone(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &Nil => Nil,
            &Cons(_, box ref tail) => tail.clone(),
        }
    }

    fn concat(&self, ys: Self) -> Self {
        match self {
            &Nil => ys,
            &Cons(ref head, box ref tail) => Cons(head.clone(), box tail.concat(ys)),
        }
    }

    fn update(&self, index: i32, x: T) -> Self {
        match self {
            &Nil => Nil,
            &Cons(ref head, ref tail) => {
                if index == 0 {
                    Cons(x, tail.clone())
                } else {
                    Cons(head.clone(), box tail.update(index - 1, x))
                }
            }
        }
    }
}

impl<T> LinkedList<T>
    where T: Debug + PartialEq + PartialOrd + Clone
{
    pub fn reverse(&self) -> Self {
        match self {
            &Nil => self.clone(),
            &Cons(ref head, box ref tail) => tail.reverse().snoc(head.clone()),
        }
    }

    fn snoc(&self, x: T) -> Self {
        match self {
            &Nil => Cons(x, box Nil),
            &Cons(ref head, ref tail) => Cons(head.clone(), box tail.snoc(x)),
        }
    }

    fn split_impl(&self, index: i32, (fore, rear): (Self, Self)) -> (Self, Self) {
        match self {
            &Nil => (fore, rear),
            &Cons(ref head, ref tail) => {
                if index % 2 == 0 {
                    tail.split_impl(index + 1, (fore.snoc(head.clone()), rear))
                } else {
                    tail.split_impl(index + 1, (fore, rear.snoc(head.clone())))
                }
            }
        }
    }

    fn split(&self) -> (Self, Self) {
        self.split_impl(0, (Nil, Nil))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let actual = LinkedList::empty().snoc(1).snoc(2).snoc(3).snoc(4).split();
        assert!(is_match_with_vec(actual.0, vec![1, 2]));
        assert!(is_match_with_vec(actual.1, vec![3, 4]));
    }


    #[test]
    fn test_reverse() {
        let actual = LinkedList::empty().snoc(1).snoc(2).snoc(3).reverse();
        assert!(is_match_with_vec(actual, vec![3, 2, 1]));
    }

    #[test]
    fn test_update() {
        let actual = LinkedList::empty().snoc(1).snoc(2).snoc(3).update(1, 4);
        assert!(is_match_with_vec(actual, vec![1, 4, 3]));
    }

    #[test]
    fn test_concat() {
        let actual_1 = LinkedList::empty().snoc(1).snoc(2).snoc(3);
        let actual_2 = LinkedList::empty().snoc(4).snoc(5).snoc(6);
        let actual = actual_1.concat(actual_2);
        assert!(is_match_with_vec(actual, vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_cons() {
        let actual = LinkedList::empty().cons(3).cons(2).cons(1);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    #[test]
    fn test_snoc() {
        let actual = LinkedList::empty().snoc(1).snoc(2).snoc(3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }
}

