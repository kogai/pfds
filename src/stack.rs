use std::fmt::Debug;

trait List<T: Clone>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;
    fn concat(&self, ys: Self) -> Self;
    fn update(&self, index: i32, x: T) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
enum Stack<T: Clone + Debug> {
    Nil,
    Node(T, Box<Stack<T>>),
}

impl<T: Clone + Debug> List<T> for Stack<T> {
    fn empty() -> Self {
        Stack::Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            &Stack::Nil => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        match self {
            &Stack::Nil => Stack::Node(x, box Stack::Nil),
            &Stack::Node(_, _) => Stack::Node(x, box self.clone()),
        }
    }

    fn head(&self) -> T {
        match self {
            &Stack::Nil => panic!("List is empty!"),
            &Stack::Node(ref elm, _) => elm.clone(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &Stack::Nil => panic!("List is empty!"),
            &Stack::Node(_, box ref next) => next.clone(),
        }
    }

    fn concat(&self, ys: Self) -> Self {
        match self {
            &Stack::Nil => ys, 
            &Stack::Node(ref head, box ref tail) => tail.concat(ys).cons(head.clone()),
        }
    }

    fn update(&self, index: i32, x: T) -> Self {
        match self {
            &Stack::Nil => panic!("List is empty!"),
            &Stack::Node(ref head, box ref tail) => {
                match index {
                    0 => tail.cons(x),
                    index => tail.update(index - 1, x).cons(head.clone()),
                }
            },
        }
    }
}

impl<T: Clone + Debug> Stack<T> {
    fn suffixes(&self) -> Stack<Self> {
        match self {
            &Stack::Nil => Stack::empty(),
            &Stack::Node(_, box ref tail) => tail.suffixes().cons(self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let list: Stack<i32> = Stack::empty();
        assert!(list.is_empty());
    }

    #[test]
    fn test_cons() {
        let actual = Stack::empty().cons(1).cons(2).cons(3);
        let expect = Stack::Node(3, box Stack::Node(2, box Stack::Node(1, box Stack::Nil)));
        assert!(actual == expect);
    }

    #[test]
    fn test_head() {
        let list = Stack::empty().cons(1);
        assert!(list.head() == 1);
    }

    #[test]
    #[should_panic]
    fn test_empty_head() {
        let list: Stack<i32> = Stack::empty();
        list.head();
    }

    #[test]
    fn test_tail() {
        let actual = Stack::empty().cons(1).cons(2).cons(3);
        let expect = Stack::Node(2, box Stack::Node(1, box Stack::Nil));
        assert!(actual.tail() == expect);
    }

    #[test]
    fn test_concat() {
        let actual = Stack::empty().cons(2).cons(1).concat(Stack::empty().cons(4).cons(3));
        let expect =
            Stack::Node(1,
                       box Stack::Node(2, box Stack::Node(3, box Stack::Node(4, box Stack::Nil))));
        assert!(actual == expect);
    }

    #[test]
    fn test_update() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).update(1, 9);
        let expect =
            Stack::Node(3,
                       box Stack::Node(9, box Stack::Node(1, box Stack::Nil)));
        assert!(actual == expect);
    }

    #[test]
    fn test_suffixes() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).suffixes();
        let expect = Stack::empty()
            .cons(Stack::empty().cons(1))
            .cons(Stack::empty().cons(1).cons(2))
            .cons(Stack::empty().cons(1).cons(2).cons(3));

        assert!(actual == expect);
    }
}
