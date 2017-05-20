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

impl<T: Ord + Clone + Debug> Tree<T> for UnBlancedTree<T> {
    fn empty() -> Self {
        UnBlancedTree::Empty
    }

    fn member(&self, x: &T) -> bool {
        match self {
            &UnBlancedTree::Empty => false,
            &UnBlancedTree::Node(ref left, ref elm, ref right) => {
                match x {
                    _ if x < elm => left.member(x),
                    _ if x > elm => right.member(x),
                    _ => true,
                }
            }
        }
    }

    fn insert(&self, x: T) -> Self {
        match self {
            &UnBlancedTree::Empty => {
                UnBlancedTree::Node(box UnBlancedTree::empty(), x, box UnBlancedTree::empty())
            }
            &UnBlancedTree::Node(ref left, ref elm, ref right) => {
                match x {
                    _ if x < *elm => {
                        UnBlancedTree::Node(box left.insert(x), elm.clone(), right.clone())
                    }
                    _ if x > *elm => {
                        UnBlancedTree::Node(left.clone(), elm.clone(), box right.insert(x))
                    }
                    _ => self.clone(),
                }
            }
        }
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
        assert!(actual.member(&15));
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
}
