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

use self::RedBlackTree::*;
use self::Color::*;

impl<T: Ord + Clone + Debug> RedBlackTree<T> {
    fn balance(color: &Color, element: &T, left: &Self, right: &Self) -> Self {
        match (color, element.clone(), left.clone(), right.clone()) {
            (&Black, ref z,
                Node {
                    color: Red,
                    element: ref y,
                    left: box Node { color: Red, element: ref x, left: box ref a, right: box ref b },
                    right: box ref c
                }, ref d
            ) | 
            (&Black, ref z,
                Node {
                    color: Red,
                    element: ref x,
                    left: box ref a,
                    right: box Node { color: Red, element: ref y, left: box ref b, right: box ref c },
                }, ref d
            ) | 
            (&Black, ref x, ref a,
                Node {
                    color: Red,
                    element: ref z,
                    left: box Node { color: Red, element: ref y, left: box ref b, right: box ref c },
                    right: box ref d
                }
            ) | 
            (&Black, ref x, ref a,
                Node {
                    color: Red,
                    element: ref y,
                    left: box ref b,
                    right: box Node { color: Red, element: ref z, left: box ref c, right: box ref d }
            }) => {
                Node {
                    color: Red,
                    element: y.clone(),
                    left: box Node {
                        color: Black,
                        element: x.clone(),
                        left: box a.clone(),
                        right: box b.clone(),
                    },
                    right: box Node {
                        color: Black,
                        element: z.clone(),
                        left: box c.clone(),
                        right: box d.clone(),
                    },
                }
            }
            _ => {
                Node {
                    color: color.clone(),
                    element: element.clone(),
                    left: box left.clone(),
                    right: box right.clone(),
                }
            }
        }
    }
    fn insert_inner(&self, x: &T) -> Self {
        match self {
            &Leaf => Node {
                color: Red,
                element: x.clone(),
                left: box Leaf,
                right: box Leaf,
            },
            &Node { ref color, ref element, box ref left, box ref right } => {
                if x < element {
                    RedBlackTree::balance(color, element, &left.insert_inner(x), right)
                } else if x > element {
                    RedBlackTree::balance(color, element, left, &right.insert_inner(x))
                } else {
                    self.clone()
                }
            } 
        }
    }

    fn from_ordered_list(xs: Vec<T>) -> Self {
        xs.iter().fold(RedBlackTree::empty(), |acc, x| acc.insert(x.clone()))
    }
}

impl<T: Ord + Clone + Debug> Set<T> for RedBlackTree<T> {
    fn empty() -> Self {
        RedBlackTree::Leaf
    }

    fn member(&self, x: &T) -> bool {
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
        if let Node { element, left, right, .. } = self.insert_inner(&x) {
            Node {
                color: Black,
                element: element,
                left: left,
                right: right,
            }
        } else {
            unreachable!();
        }
 
    }
}

mod tests {
    use super::*;

    fn is_red_has_black<T: Ord + Clone + Debug>(this: &RedBlackTree<T>) -> bool {
        match this {
            &Node {
                color: Red,
                left: box Node { color: Red, .. },
                ..
            } | &Node {
                color: Red,
                right: box Node { color: Red, .. },
                ..
            } => false,
            _ => true
        }
    }

    fn count_black<T: Ord + Clone + Debug>(this: &RedBlackTree<T>, count: i32) -> Vec<i32> {
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
    fn test_from_ordered_list() {
        let actual = RedBlackTree::from_ordered_list(vec![1, 2, 3, 4, 5, 6, 7]);
        
        assert!(is_red_has_black(&actual));
        assert!(has_same_blacks(&actual));
    }

    #[test]
    fn test_insert() {
        let actual = RedBlackTree::empty()
            .insert(1)
            .insert(2)
            .insert(3)
            ;

        assert!(is_red_has_black(&actual));
        assert!(has_same_blacks(&actual));
    }

    #[test]
    fn test_member() {
        let actual = RedBlackTree::empty().insert(1).insert(3).insert(5);
        assert!(actual.member(&1));
        assert!(!actual.member(&2));
    }
}
