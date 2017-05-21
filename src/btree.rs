use std::fmt::Debug;

trait Tree<T: Ord + Clone + Debug> {
    fn empty() -> Self;
    fn member(&self, x: &T) -> bool;
    fn insert(&self, x: T) -> Self;
}

#[derive(PartialEq, Debug, Clone)]
enum UnBlancedTree<T: Ord + Clone + Debug> {
    Empty,
    Node(Box<UnBlancedTree<T>>, T, Box<UnBlancedTree<T>>),
}

impl<T: Ord + Clone + Debug> UnBlancedTree<T> {
    fn member_inner(&self, x: &T, parent: Option<&T>) -> bool {
        match self {
            &UnBlancedTree::Empty => {
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
            &UnBlancedTree::Empty => {
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
}

impl<T: Ord + Clone + Debug> Tree<T> for UnBlancedTree<T> {
    fn empty() -> Self {
        UnBlancedTree::Empty
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

    #[test]
    fn test_empty() {
        let actual: UnBlancedTree<i32> = UnBlancedTree::empty();
        assert!(actual == UnBlancedTree::Empty);
    }

    #[test]
    fn test_member() {
        let actual = UnBlancedTree::Node(
          box UnBlancedTree::Node(
            box UnBlancedTree::Node(
              box UnBlancedTree::Empty,
              3,
              box UnBlancedTree::Empty,
            ),
            5,
            box UnBlancedTree::Empty,
          ),
          10,
          box UnBlancedTree::Node(
            box UnBlancedTree::Empty,
            15,
            box UnBlancedTree::Node(
              box UnBlancedTree::Empty,
              20,
              box UnBlancedTree::Empty,
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
        let actual = UnBlancedTree::empty().insert(10).insert(5).insert(3).insert(15).insert(20);
        let expect = UnBlancedTree::Node(
          box UnBlancedTree::Node(
            box UnBlancedTree::Node(
              box UnBlancedTree::Empty,
              3,
              box UnBlancedTree::Empty,
            ),
            5,
            box UnBlancedTree::Empty,
          ),
          10,
          box UnBlancedTree::Node(
            box UnBlancedTree::Empty,
            15,
            box UnBlancedTree::Node(
              box UnBlancedTree::Empty,
              20,
              box UnBlancedTree::Empty,
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
}
