use std::fmt::Debug;
use lazy::Susp;

use self::StreamCell::*;

#[derive(Debug, PartialEq, Clone)]
enum StreamCell<'a, T: 'a + Debug + PartialEq + PartialOrd + Clone> {
    Nil,
    Cons(T, Box<Stream<'a, T>>),
}

impl<'a, T: 'a + Debug + PartialEq + PartialOrd + Clone> StreamCell<'a, T> {
    fn drop_impl(&self, n: i32) -> Self {
        match self {
            &Nil => Nil,
            _ if n == 0 => self.clone(),
            &Cons(_, box ref tail) => (**tail).drop_impl(n - 1),
        }
    }

    fn reverse_impl(&self, other: &Self) -> Self {
        let that = other.clone();
        match (self, other) {
            (&Nil, _) => other.clone(),
            (&Cons(ref head, box ref tail), _) => {
                let last = Cons(head.clone(), box susp!(that.clone()));
                (**tail).reverse_impl(&last)
            }
        }
    }

    fn insert(&self, x: &T) -> Self {
        match self {
            &Nil => Cons(x.clone(), box Stream::empty()),
            &Cons(ref head, box ref tail) => {
                if x < head {
                    let head = head.clone();
                    let tail = tail.clone();
                    Cons(x.clone(), box susp!(tail.insert(&head)))
                } else {
                    let tail = tail.clone();
                    let x = x.clone();
                    Cons(head.clone(), box susp!(tail.insert(&x)))
                }
            }
        }
    }

    fn insert_sort_impl(&self, sorted: &Self, x: &T) -> Self {
        match self {
            &Nil => sorted.clone(),
            &Cons(ref head, ref tail) => {
                if x < head {
                    tail.insert_sort_impl(&sorted.insert(x), head)
                } else {
                    tail.insert_sort_impl(&sorted.insert(head), x)
                }
            }
        }
    }
}

type Stream<'a, T: 'a + Debug + PartialEq + PartialOrd + Clone> = Susp<'a, StreamCell<'a, T>>;

impl<'a, T: Debug + PartialEq + PartialOrd + Clone> Stream<'a, T> {
    fn empty() -> Self {
        susp!(Nil)
    }

    fn cons(&self, x: &T) -> Self {
        let this = self.clone();
        let x = x.clone();
        susp!({
                  match *this {
                      Nil => Cons(x.clone(), box Stream::empty()),
                      Cons(ref head, ref tail) => Cons(head.clone(), box tail.cons(&x)),
                  }
              })
    }

    fn concat(&self, other: &Self) -> Self {
        let this = self.clone();
        let that = other.clone();
        susp!({
                  match *this {
                      Nil => (*that).clone(),
                      Cons(ref head, ref tail) => Cons(head.clone(), box tail.concat(&that)),
                  }
              })
    }

    fn take(&self, n: i32) -> Self {
        let this = self.clone();
        susp!({
                  match *this {
                      Nil => Nil,
                      _ if n == 0 => Nil,
                      Cons(ref head, ref tail) => Cons(head.clone(), box tail.take(n - 1)),
                  }
              })
    }

    fn drop_nth(&self, n: i32) -> Self {
        let this = self.clone();
        susp!((*this).drop_impl(n))
    }

    fn reverse(&self) -> Self {
        let this = self.clone();
        susp!((*this).reverse_impl(&Nil))
    }

    fn insert_sort(&self) -> Self {
        let this = self.clone();
        susp!({
                  match &*this.take(1) {
                      &Nil => Nil,
                      &Cons(ref head, _) => (*this).insert_sort_impl(&Nil, &head.clone()),
                  }
              })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_insert_sort() {
        let actual = Stream::empty().cons(&3).cons(&1).cons(&2).insert_sort();
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    #[test]
    fn test_reverse() {
        let actual = Stream::empty().cons(&1).cons(&2).cons(&3).reverse();
        assert!(is_match_with_vec(actual, vec![3, 2, 1]));
    }

    #[test]
    fn test_drop() {
        let actual = Stream::empty().cons(&1).cons(&2).cons(&3).drop_nth(2);
        assert!(is_match_with_vec(actual, vec![3]));
    }

    #[test]
    fn test_take() {
        let actual = Stream::empty().cons(&1).cons(&2).cons(&3).take(2);
        assert!(is_match_with_vec(actual, vec![1, 2]));
    }

    #[test]
    fn test_cons() {
        let actual = Stream::empty().cons(&1).cons(&2).cons(&3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    #[test]
    fn test_concat() {
        let actual_1 = Stream::empty().cons(&1).cons(&2).cons(&3);
        let actual_2 = Stream::empty().cons(&4).cons(&5).cons(&6);

        let actual = actual_1.concat(&actual_2);
        assert!(is_match_with_vec(actual, vec![1, 2, 3, 4, 5, 6]));
    }

    fn is_match_with_vec<'a, T>(xs: Stream<'a, T>, ys: Vec<T>) -> bool
        where T: 'a + Debug + PartialEq + PartialOrd + Clone
    {
        ys.iter()
            .fold((xs, true), |(xs, prev), y| match *xs {
                Nil => (susp!(Nil), false),
                Cons(ref head, box ref tail) => (tail.clone(), prev && head == y),
            })
            .1
    }
}

