use std::fmt::Debug;
use set::Set;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
enum RedBlackTree<T: Ord + Clone + Debug> {
    Leaf,
    Node {
        color: Color,
        element: T,
        left: Box<RedBlackTree<T>>,
        right: Box<RedBlackTree<T>>,
    },
}

impl<T: Ord + Clone + Debug> RedBlackTree<T> {
    fn color(&self) -> Color {
        use self::RedBlackTree::*;
        match self {
            &Leaf => Color::Black, 
            &Node { ref color, .. } => color.clone(),
        }
    }

    fn balance(&self) -> Self {
        use self::RedBlackTree::*;
        match self {
            &Leaf => unreachable!(),
            &Node { ref color, ref element, ref left, ref right } => {
                unimplemented!();
            }
        }
    }
}

impl<T: Ord + Clone + Debug> Set<T> for RedBlackTree<T> {
    fn empty() -> Self {
        RedBlackTree::Leaf
    }

    fn member(&self, x: &T) -> bool {
        use self::RedBlackTree::*;
        match self {
            &Leaf => false,
            &Node { ref element, ref left, ref right, .. } => {
                if x < element {
                    return left.member(x);
                }
                if x > element {
                    return right.member(x);
                }
                true
            }
        }
    }

    fn insert(&self, x: T) -> Self {
        use self::RedBlackTree::*;
        match self {
            &Leaf => {
                Node {
                    color: Color::Red,
                    element: x,
                    left: box Leaf,
                    right: box Leaf,
                }
            }
            &Node { ref element, ref left, ref right, .. } => {
                if x < *element {
                    return left.insert(x).balance();
                }
                if x > *element {
                    return right.insert(x).balance();
                }
                self.clone()
            } 
        }
    }
}

mod tests {
    use super::*;

    fn is_red_has_black<T: Ord + Clone + Debug>(this: &RedBlackTree<T>) -> bool {
        use self::RedBlackTree::*;
        match this {
            &Leaf => true,
            &Node { ref color, ref left, ref right, .. } => {
                *color != left.color() && *color != right.color() && is_red_has_black(left) &&
                is_red_has_black(right)
            }
        }
    }

    fn count_black<T: Ord + Clone + Debug>(this: &RedBlackTree<T>, count: i32) -> Vec<i32> {
        use self::RedBlackTree::*;
        match this {
            &Leaf => vec![count + 1],
            &Node { ref color, ref left, ref right, .. } => {
                match color {
                    &Color::Black => {
                        [count_black(left, count + 1).as_slice(),
                         count_black(right, count + 1).as_slice()]
                            .concat()
                    }
                    &Color::Red => {
                        [count_black(left, count).as_slice(), count_black(right, count).as_slice()]
                            .concat()
                    }
                }
            }
        }
    }

    fn has_same_blacks<T: Ord + Clone + Debug>(this: &RedBlackTree<T>) -> bool {
        let blacks = count_black(this, 0);
        match blacks.first() {
            Some(count) => blacks.iter().all(|x| x == count),
            None => true,
        }
    }

    #[test]
    fn test_insert() {
        let actual = RedBlackTree::empty().insert(1);
        assert!(is_red_has_black(&actual));
        assert!(has_same_blacks(&actual));
    }

    #[test]
    fn test_member() {
        let actual = RedBlackTree::empty().insert(1);
        assert!(actual.member(&1));
        assert!(!actual.member(&2));
    }
}
