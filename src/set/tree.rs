use std::fmt::Debug;
use set::{Set, Sequence};

#[derive(PartialEq, Debug, Clone)]
enum UnBalancedTree<T: Ord + Clone + Debug + Sequence> {
    Leaf,
    Node(Box<UnBalancedTree<T>>, T, Box<UnBalancedTree<T>>),
}

use self::UnBalancedTree::*;

impl<T: Ord + Clone + Debug + Sequence> UnBalancedTree<T> {
    fn member_inner(&self, x: &T, parent: Option<&T>) -> bool {
        match self {
            &Leaf => {
                match parent {
                    Some(p) => x <= p,
                    None => false,
                }
            }
            &Node(ref left, ref elm, ref right) => {
                if x < elm {
                    left.member_inner(x, parent)
                } else {
                    right.member_inner(x, Some(elm))
                }
            }
        }
    }

    fn insert_inner(&self, x: T, cache: Option<T>) -> Self {
        match self {
            &Leaf => {
                if let Some(c) = cache {
                    // 自動導出されたclone関数は参照をコピーする https://doc.rust-lang.org/src/core/clone.rs.html#134
                    if x <= c {
                        return self.clone();
                    }
                }
                Node(box UnBalancedTree::empty(), x, box UnBalancedTree::empty())
            }
            &Node(ref left, ref elm, ref right) => {
                if x < *elm {
                    Node(box left.insert_inner(x, cache), elm.clone(), right.clone())
                } else {
                    Node(left.clone(),
                         elm.clone(),
                         box right.insert_inner(x, Some(elm.clone())))
                }
            }
        }
    }

    fn complete(x: T, d: i32) -> Self {
        match d {
            1 => UnBalancedTree::empty().insert(x),
            _ => {
                Node(box UnBalancedTree::complete(x.clone(), d - 1),
                     x.clone(),
                     box UnBalancedTree::complete(x.clone(), d - 1))
            }
        }
    }

    fn create(x: T, d: i32) -> Self {
        match d {
            1 => UnBalancedTree::empty().insert(x),
            _ => {
                Node(box UnBalancedTree::create(x.to_predecessor_with(d - 1), d - 1),
                     x.clone(),
                     box UnBalancedTree::create(x.to_successor_with(d - 1), d - 1))
            }
        }
    }
}

impl<T: Ord + Clone + Debug + Sequence> Set<T> for UnBalancedTree<T> {
    fn empty() -> Self {
        Leaf
    }

    fn member(&self, x: &T) -> bool {
        self.member_inner(x, None)
    }

    fn insert(&self, x: T) -> Self {
        self.insert_inner(x, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Sequence for i32 {
        fn to_successor_with(&self, skip: i32) -> Self {
            self + skip
        }
        fn to_predecessor_with(&self, skip: i32) -> Self {
            self - skip
        }
    }

    #[test]
    fn test_empty() {
        let actual: UnBalancedTree<i32> = UnBalancedTree::empty();
        assert!(actual == Leaf);
    }

    #[test]
    fn test_member() {
        let actual = Node(box Node(box Node(box Leaf, 3, box Leaf), 5, box Leaf),
                          10,
                          box Node(box Leaf, 15, box Node(box Leaf, 20, box Leaf)));

        assert!(!actual.member(&1));
        assert!(!actual.member(&17));
        assert!(actual.member(&3));
        assert!(actual.member(&5));
        assert!(actual.member(&15));
        assert!(actual.member(&20));
    }

    #[test]
    fn test_insert() {
        let actual = UnBalancedTree::empty()
            .insert(10)
            .insert(5)
            .insert(3)
            .insert(15)
            .insert(20);
        let expect = Node(box Node(box Node(box Leaf, 3, box Leaf), 5, box Leaf),
                          10,
                          box Node(box Leaf, 15, box Node(box Leaf, 20, box Leaf)));

        assert!(actual == expect);
    }

    #[test]
    fn test_insert_ref_equality() {
        let actual = UnBalancedTree::empty().insert(10);
        assert!(&actual == &actual.insert(10));
        assert!(&actual != &actual.insert(11));
    }

    #[test]
    fn test_complete() {
        let actual = UnBalancedTree::complete(10, 3);
        let expect = Node(box Node(box Node(box Leaf, 10, box Leaf),
                                   10,
                                   box Node(box Leaf, 10, box Leaf)),
                          10,
                          box Node(box Node(box Leaf, 10, box Leaf),
                                   10,
                                   box Node(box Leaf, 10, box Leaf)));
        assert!(actual == expect);
    }

    #[test]
    fn test_create() {
        let actual = UnBalancedTree::create(10, 3);
        let expect = Node(box Node(box Node(box Leaf, 7, box Leaf),
                                   8,
                                   box Node(box Leaf, 9, box Leaf)),
                          10,
                          box Node(box Node(box Leaf, 11, box Leaf),
                                   12,
                                   box Node(box Leaf, 13, box Leaf)));
        assert!(actual == expect);
    }
}

