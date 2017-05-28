use std::fmt::Debug;
use set::Set;

#[derive(Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
enum RedBlackTree<T: Ord + Clone + Debug> {
    Leaf,
    Node {
        color: Color,
        element: T,
        left: Box<RedBlackTree<T>>,
        right: Box<RedBlackTree<T>>,
    },
}

mod tests {
    use super::*;

    fn is_red_has_black<T: Ord + Clone + Debug>(x: &RedBlackTree<T>) -> bool {
        unimplemented!();
    }

    fn has_same_blacks<T: Ord + Clone + Debug>(x: &RedBlackTree<T>) -> bool {
        unimplemented!();
    }

    #[test]
    fn test_ok() {
        unimplemented!();
    }
}

