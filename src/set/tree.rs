use std::fmt::Debug;
use set::{Set, Sequence};

#[derive(PartialEq, Debug, Clone)]
enum UnBlancedTree<T: Ord + Clone + Debug + Sequence> {
    Leaf,
    Node(Box<UnBlancedTree<T>>, T, Box<UnBlancedTree<T>>),
}

impl<T: Ord + Clone + Debug + Sequence> UnBlancedTree<T> {
    fn member_inner(&self, x: &T, parent: Option<&T>) -> bool {
        match self {
            &UnBlancedTree::Leaf => {
                match parent {
                    Some(p) => x <= p,
                    None => false,
                }
            }
            &UnBlancedTree::Node(ref left, ref elm, ref right) => {
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
            &UnBlancedTree::Leaf => {
                if let Some(c) = cache {
                    // 自動導出されたclone関数は参照をコピーする https://doc.rust-lang.org/src/core/clone.rs.html#134
                    if x <= c {
                        return self.clone();
                    }
                }
                UnBlancedTree::Node(box UnBlancedTree::empty(), x, box UnBlancedTree::empty())
            }
            &UnBlancedTree::Node(ref left, ref elm, ref right) => {
                if x < *elm {
                    UnBlancedTree::Node(box left.insert_inner(x, cache), elm.clone(), right.clone())
                } else {
                    UnBlancedTree::Node(left.clone(),
                                        elm.clone(),
                                        box right.insert_inner(x, Some(elm.clone())))
                }
            }
        }
    }

    fn complete(x: T, d: i32) -> Self {
        match d {
            1 => UnBlancedTree::empty().insert(x),
            _ => {
                UnBlancedTree::Node(box UnBlancedTree::complete(x.clone(), d - 1),
                                    x.clone(),
                                    box UnBlancedTree::complete(x.clone(), d - 1))
            }
        }
    }

    fn create(x: T, d: i32) -> Self {
        match d {
            1 => UnBlancedTree::empty().insert(x),
            _ => {
                UnBlancedTree::Node(box UnBlancedTree::create(x.to_predecessor_with(d - 1), d - 1),
                                    x.clone(),
                                    box UnBlancedTree::create(x.to_successor_with(d - 1), d - 1))
            }
        }
    }
}

impl<T: Ord + Clone + Debug + Sequence> Set<T> for UnBlancedTree<T> {
    fn empty() -> Self {
        UnBlancedTree::Leaf
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
        let actual: UnBlancedTree<i32> = UnBlancedTree::empty();
        assert!(actual == UnBlancedTree::Leaf);
    }

    #[test]
    fn test_member() {
        let actual = UnBlancedTree::Node(
          box UnBlancedTree::Node(
            box UnBlancedTree::Node(
              box UnBlancedTree::Leaf,
              3,
              box UnBlancedTree::Leaf,
            ),
            5,
            box UnBlancedTree::Leaf,
          ),
          10,
          box UnBlancedTree::Node(
            box UnBlancedTree::Leaf,
            15,
            box UnBlancedTree::Node(
              box UnBlancedTree::Leaf,
              20,
              box UnBlancedTree::Leaf,
            ),
          ),
        );

        assert!(!actual.member(&1));
        assert!(!actual.member(&17));
        assert!(actual.member(&3));
        assert!(actual.member(&5));
        assert!(actual.member(&15));
        assert!(actual.member(&20));
    }

    #[test]
    fn test_insert() {
        let actual = UnBlancedTree::empty()
            .insert(10)
            .insert(5)
            .insert(3)
            .insert(15)
            .insert(20);
        let expect = UnBlancedTree::Node(
          box UnBlancedTree::Node(
            box UnBlancedTree::Node(
              box UnBlancedTree::Leaf,
              3,
              box UnBlancedTree::Leaf,
            ),
            5,
            box UnBlancedTree::Leaf,
          ),
          10,
          box UnBlancedTree::Node(
            box UnBlancedTree::Leaf,
            15,
            box UnBlancedTree::Node(
              box UnBlancedTree::Leaf,
              20,
              box UnBlancedTree::Leaf,
            ),
          ),
        );

        assert!(actual == expect);
    }

    #[test]
    fn test_insert_ref_equality() {
        let actual = UnBlancedTree::empty().insert(10);
        assert!(&actual == &actual.insert(10));
        assert!(&actual != &actual.insert(11));
    }

    #[test]
    fn test_complete() {
        let actual = UnBlancedTree::complete(10, 3);
        let expect = UnBlancedTree::Node(
            box UnBlancedTree::Node(
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    10,
                    box UnBlancedTree::Leaf,
                ),
                10,
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    10,
                    box UnBlancedTree::Leaf,
                ),
            ),
            10,
            box UnBlancedTree::Node(
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    10,
                    box UnBlancedTree::Leaf,
                ),
                10,
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    10,
                    box UnBlancedTree::Leaf,
                ),
            ),
        );
        assert!(actual == expect);
    }

    #[test]
    fn test_create() {
        let actual = UnBlancedTree::create(10, 3);
        let expect = UnBlancedTree::Node(
            box UnBlancedTree::Node(
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    7,
                    box UnBlancedTree::Leaf,
                ),
                8,
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    9,
                    box UnBlancedTree::Leaf,
                ),
            ),
            10,
            box UnBlancedTree::Node(
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    11,
                    box UnBlancedTree::Leaf,
                ),
                12,
                box UnBlancedTree::Node(
                    box UnBlancedTree::Leaf,
                    13,
                    box UnBlancedTree::Leaf,
                ),
            ),
        );
        assert!(actual == expect);
    }
}

