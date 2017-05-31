use std::fmt::Debug;
use list::List;

#[derive(Debug, PartialEq, Clone)]
pub enum Stack<T: Clone + Debug> {
    Nil,
    Cell(T, Box<Stack<T>>),
}

use self::Stack::*;

impl<T: Clone + Debug> List<T> for Stack<T> {
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
            &Nil => Cell(x, box Nil),
            &Cell(_, _) => Cell(x, box self.clone()),
        }
    }

    fn head(&self) -> T {
        match self {
            &Nil => panic!("List is empty!"),
            &Cell(ref elm, _) => elm.clone(),
        }
    }

    fn tail(&self) -> Self {
        match self {
            &Nil => panic!("List is empty!"),
            &Cell(_, box ref next) => next.clone(),
        }
    }

    fn concat(&self, ys: Self) -> Self {
        match self {
            &Nil => ys, 
            &Cell(ref head, box ref tail) => tail.concat(ys).cons(head.clone()),
        }
    }

    fn update(&self, index: i32, x: T) -> Self {
        match self {
            &Nil => panic!("List is empty!"),
            &Cell(ref head, box ref tail) => {
                match index {
                    0 => tail.cons(x),
                    index => tail.update(index - 1, x).cons(head.clone()),
                }
            }
        }
    }
}

impl<T: Clone + Debug> Stack<T> {
    pub fn new(x: T) -> Self {
        Cell(x, box Nil)
    }

    fn suffixes(&self) -> Stack<Self> {
        match self {
            &Nil => Stack::empty(),
            &Cell(_, box ref tail) => tail.suffixes().cons(self.clone()),
        }
    }

    pub fn map<R, F>(&self, f: &F) -> Stack<R>
        where R: Clone + Debug,
              F: Fn(&T) -> R
    {
        match self {
            &Nil => Nil,
            &Cell(ref head, ref tail) => tail.map(f).cons(f(head)),
        }
    }

    pub fn foldl<R, F>(&self, r: R, f: &F) -> R
        where R: Clone + Debug,
              F: Fn(R, &T) -> R
    {
        match self {
            &Nil => r.clone(),
            &Cell(ref head, ref tail) => tail.foldl(f(r, head), f), 
        }
    }

    pub fn all<F>(&self, f: &F) -> bool
        where F: Fn(&T) -> bool
    {
        self.foldl(true, &|acc, x| acc && f(x))
    }

    pub fn reverse(&self) -> Self {
        match self {
            &Nil => self.clone(),
            &Cell(ref head, box ref tail) => {
                if tail.is_empty() {
                    Stack::new(head.clone())
                } else {
                    tail.reverse().concat(Stack::new(head.clone()))
                }
            } 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).reverse();
        let expect = Cell(1, box Cell(2, box Cell(3, box Nil)));
        assert!(actual.head() == expect.head());
    }

    #[test]
    fn test_map() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).map(&|x| x + 1);
        let expect = Cell(4, box Cell(3, box Cell(2, box Nil)));
        assert!(actual == expect);
    }

    #[test]
    fn test_foldl() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).foldl(0, &|acc, x| acc + x);
        assert!(actual == 6);
    }

    #[test]
    fn test_all() {
        let actual = Stack::empty().cons(2).cons(4).cons(6).all(&|x| x % 2 == 0);
        assert!(actual);
    }

    #[test]
    fn test_is_empty() {
        let list: Stack<i32> = Stack::empty();
        assert!(list.is_empty());
    }

    #[test]
    fn test_cons() {
        let actual = Stack::empty().cons(1).cons(2).cons(3);
        let expect = Cell(3, box Cell(2, box Cell(1, box Nil)));
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
        let expect = Cell(2, box Cell(1, box Nil));
        assert!(actual.tail() == expect);
    }

    #[test]
    fn test_concat() {
        let actual = Stack::empty().cons(2).cons(1).concat(Stack::empty().cons(4).cons(3));
        let expect =
            Cell(1,
                        box Cell(2, box Cell(3, box Cell(4, box Nil))));
        assert!(actual == expect);
    }

    #[test]
    fn test_update() {
        let actual = Stack::empty().cons(1).cons(2).cons(3).update(1, 9);
        let expect = Cell(3, box Cell(9, box Cell(1, box Nil)));
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
