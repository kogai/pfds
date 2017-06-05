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

    fn len_impl(&self, l: i32) -> i32 {
        match self {
            &Nil => l,
            &Cons(_, ref tail) => tail.len_impl(l + 1),
        }
    }

    pub fn len(&self) -> i32 {
        self.len_impl(0)
    }

    fn median(&self) -> i32 {
        match self.len() {
            0 => unreachable!(),
            n => (n as f32 / 2.0).round() as i32,
        }
    }

    pub fn take(&self, n: i32) -> Self {
        match self {
            &Nil => Nil,
            _ if n == 0 => Nil,
            &Cons(ref head, ref tail) => Cons(head.clone(), box tail.take(n - 1)),
        }
    }

    pub fn drop_nth(&self, n: i32) -> Self {
        match self {
            &Nil => self.clone(),
            _ if n == 0 => self.clone(),
            &Cons(_, box ref tail) => tail.drop_nth(n - 1),
        }
    }

    pub fn init(&self) -> Self {
        self.take(self.len() - 1)
    }

    pub fn snoc(&self, x: T) -> Self {
        match self {
            &Nil => Cons(x, box Nil),
            &Cons(ref head, ref tail) => Cons(head.clone(), box tail.snoc(x)),
        }
    }

    pub fn split(&self) -> (Self, Self) {
        match self {
            &Nil => (Nil, Nil),
            &Cons(_, _) => {
                let median = self.median();
                (self.take(median), self.drop_nth(median))
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        let actual = LinkedList::empty().cons(1);
        assert!(is_match_with_vec(actual.drop_nth(0), vec![1]));
        assert!(is_match_with_vec(actual.drop_nth(1), vec![]));
    }

    #[test]
    fn test_take() {
        let actual = LinkedList::empty().cons(1);
        assert!(is_match_with_vec(actual.take(0), vec![]));
        assert!(is_match_with_vec(actual.take(1), vec![1]));
    }

    #[test]
    fn test_split() {
        let actual = LinkedList::empty().snoc(1).snoc(2).snoc(3).split();
        assert!(is_match_with_vec(actual.0, vec![1, 2]));
        assert!(is_match_with_vec(actual.1, vec![3]));

        let actual = LinkedList::empty().cons(1).split();
        assert!(is_match_with_vec(actual.0, vec![1]));
        assert!(is_match_with_vec(actual.1, vec![]));
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

