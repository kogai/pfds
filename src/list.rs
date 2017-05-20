trait Stack<T: Clone>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
enum List<T: Clone> {
    Nil,
    Node(T, Box<List<T>>),
}

impl<T: Clone> Stack<T> for List<T> {
    fn empty() -> Self {
        List::Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            &List::Nil => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        match self {
            &List::Nil => List::Node(x, box List::Nil),
            &List::Node(ref elm, ref next) => List::Node(elm.clone(), box next.cons(x)),
        }
    }

    fn head(&self) -> T {
        match self {
            &List::Nil => panic!("List is empty!"),
            &List::Node(ref elm, _) => elm.clone(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &List::Nil => panic!("List is empty!"),
            &List::Node(_, box ref next) => next.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let list: List<i32> = List::empty();
        assert!(list.is_empty());
    }

    #[test]
    fn test_cons() {
        let actual = List::empty().cons(1).cons(2).cons(3);
        let expect = List::Node(1, box List::Node(2, box List::Node(3, box List::Nil)));
        assert!(actual == expect);
    }

    #[test]
    fn test_head() {
        let list = List::empty().cons(1);
        assert!(list.head() == 1);
    }

    #[test]
    #[should_panic]
    fn test_empty_head() {
        let list: List<i32> = List::empty();
        list.head();
    }

    #[test]
    fn test_tail() {
        let actual = List::empty().cons(1).cons(2).cons(3);
        let expect = List::Node(2, box List::Node(3, box List::Nil));
        assert!(actual.tail() == expect);
    }
}
