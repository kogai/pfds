use std::fmt::Debug;
use lazy::Susp;

use self::StreamCell::*;

#[derive(Debug, PartialEq, Clone)]
enum StreamCell<'a, T: 'a + Debug + PartialEq + Clone> {
    Nil,
    Cons(T, Box<Stream<'a, T>>),
}

type Stream<'a, T: 'a + Debug + PartialEq + Clone> = Susp<'a, StreamCell<'a, T>>;

impl<'a, T: 'a + Debug + PartialEq + Clone> Stream<'a, T> {
    fn empty() -> Self {
        susp!(Nil)
    }

    fn concat(&self, other: &Self) -> Self {
        match **self {
            Nil => other.clone(),
            Cons(ref head, ref body) => susp!(Cons(head.clone(), box body.concat(other))),
        }
    }

    fn take(&self, n: i32) -> Self {
        unimplemented!();
    }

    fn drop(&self, n: i32) -> Self {
        unimplemented!();
    }

    fn reverse(&self) -> Self {
        unimplemented!();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        let actual = susp!(Cons(1, box Stream::empty()))
            .concat(&susp!(Cons(2, box Stream::empty())));
        println!("{:?}", *actual);
        // unimplemented!();
    }
}

