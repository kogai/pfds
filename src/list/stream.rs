use std::fmt::Debug;
use lazy::{Susp, Thunk};

use self::StreamCell::*;

#[derive(Debug, PartialEq, Clone)]
enum StreamCell<'a, T: 'a + Debug + PartialEq + Clone> {
    Nil,
    Cons(T, Box<Stream<'a, T>>),
}

type Stream<'a, T: 'a + Debug + PartialEq + Clone> = Susp<'a, StreamCell<'a, T>>;

impl<'a, T: Debug + PartialEq + Clone> Stream<'a, T> {
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
        match self.unwrap() {
            Nil => other.clone(),
            Cons(ref head, ref tail) => {
                let stream = Cons(head.clone(), box tail.concat(other));
                susp!(stream.clone())
            }
        }
    }

    /*
    fn take(&self, n: i32) -> Self {
        unimplemented!();
    }

    fn drop(&self, n: i32) -> Self {
        unimplemented!();
    }

    fn reverse(&self) -> Self {
        unimplemented!();
    }
    */
}

mod tests {
    use super::*;

    fn is_match_with_vec<'a, T: 'a + Debug + PartialEq + Clone>(xs: Stream<'a, T>,
                                                                ys: Vec<T>)
                                                                -> bool {
        ys.iter()
            .fold((xs, true), |(xs, prev), y| match *xs {
                Nil => (susp!(Nil), false),
                Cons(ref head, box ref tail) => (tail.clone(), prev && head == y),
            })
            .1
    }

    #[test]
    fn test_cons() {
        let actual = Stream::empty().cons(&1).cons(&2).cons(&3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    #[test]
    fn test_concat() {
        let actual_1 = susp!(Cons(1, box Stream::empty()));
        let actual_2 = susp!(Cons(2, box Stream::empty()));

        let actual = actual_1.concat(&actual_2);
        assert!(is_match_with_vec(actual, vec![1, 2]));
    }
}

