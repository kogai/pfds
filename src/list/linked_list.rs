use std::fmt::Debug;
use list::List;

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
            &Cons(ref head, ref tail) => Cons(head.clone(), box tail.cons(x)),
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
            &Cons(ref head, box ref tail) => tail.reverse().cons(head.clone()),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let actual = LinkedList::empty().cons(1).cons(2).cons(3).reverse();
        assert!(is_match_with_vec(actual, vec![3, 2, 1]));
    }

    #[test]
    fn test_update() {
        let actual = LinkedList::empty().cons(1).cons(2).cons(3).update(1, 4);
        assert!(is_match_with_vec(actual, vec![1, 4, 3]));
    }

    #[test]
    fn test_concat() {
        let actual_1 = LinkedList::empty().cons(1).cons(2).cons(3);
        let actual_2 = LinkedList::empty().cons(4).cons(5).cons(6);
        let actual = actual_1.concat(actual_2);
        assert!(is_match_with_vec(actual, vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_cons() {
        let actual = LinkedList::empty().cons(1).cons(2).cons(3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    fn is_match_with_vec<T, L>(xs: L, ys: Vec<T>) -> bool
        where T: Debug + PartialEq + PartialOrd + Clone,
              L: List<T>
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

