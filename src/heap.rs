trait Heap<T: Clone> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn insert(&self, x: T) -> Self;
    fn merge(&self, x: Self) -> Self;
    fn find_min(&self) -> Option<T>;
    fn delete_min(&self) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
struct LeftistHeapElement<T: Clone> {
    rank: i32,
    element: T,
    left: Box<LeftistHeap<T>>,
    right: Box<LeftistHeap<T>>,
}

impl<T: Clone> LeftistHeapElement<T> {
    fn make_tree(&self, other: &Self, x: T) -> Self {
        // x < self.element && x < other.element が前提
        // ランクの低い方の部分木を元に根のランクを算出し、右ノードに生やす
        if self.rank < other.rank {
            LeftistHeapElement {
                rank: self.rank + 1,
                element: x,
                left: box LeftistHeap::Element(other.clone()),
                right: box LeftistHeap::Element(self.clone()),
            }
        } else {
            LeftistHeapElement {
                rank: other.rank + 1,
                element: x,
                left: box LeftistHeap::Element(self.clone()),
                right: box LeftistHeap::Element(other.clone()),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum LeftistHeap<T: Clone> {
    Nil,
    Element(LeftistHeapElement<T>),
}

impl<T: Clone> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        LeftistHeap::Nil
    }
    fn is_empty(&self) -> bool {
        match self {
            &LeftistHeap::Nil => true,
            _ => false,
        }
    }
    fn insert(&self, x: T) -> Self {
        unimplemented!();
    }
    fn merge(&self, x: Self) -> Self {
        unimplemented!();
    }
    fn find_min(&self) -> Option<T> {
        unimplemented!();
    }
    fn delete_min(&self) -> Self {
        unimplemented!();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_make_tree() {
        let actual_this = LeftistHeapElement {
            rank: 1,
            element: 10,
            left: box LeftistHeap::empty(),
            right: box LeftistHeap::empty(),
        };
        let actual_other = LeftistHeapElement {
            rank: 1,
            element: 20,
            left: box LeftistHeap::empty(),
            right: box LeftistHeap::empty(),
        };

        let actual = actual_this.make_tree(&actual_other, 5);
        assert!(actual ==
                LeftistHeapElement {
                    rank: 2,
                    element: 5,
                    left: box LeftistHeap::Element(actual_this.clone()),
                    right: box LeftistHeap::Element(actual_other.clone()),
                });

        assert!(actual_this.clone().make_tree(&actual.clone(), 1) ==
                LeftistHeapElement {
                    rank: 2,
                    element: 1,
                    left: box LeftistHeap::Element(actual.clone()),
                    right: box LeftistHeap::Element(actual_this.clone()),
                });
    }
}

