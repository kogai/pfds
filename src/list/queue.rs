use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
struct BatchedQueue<T: Clone + PartialEq + Debug> {
    fore: Vec<T>,
    rear: Vec<T>,
}

impl<T> BatchedQueue<T>
    where T: Clone + PartialEq + Debug
{
    fn empty() -> Self {
        BatchedQueue {
            fore: vec![],
            rear: vec![],
        }
    }

    fn is_empty(&self) -> bool {
        self.fore.is_empty()
    }

    fn snoc(&self, x: &T) -> Self {
        unimplemented!();
    }

    fn head(&self) -> T {
        unimplemented!();
    }

    fn tail(&self) -> Self {
        unimplemented!();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sonc() {
        let actual = BatchedQueue::empty().snoc(&1).snoc(&2).snoc(&3);
        assert!(is_match_with_vec(actual, vec![1, 2, 3]));
    }

    fn is_match_with_vec<T>(xs: BatchedQueue<T>, ys: Vec<T>) -> bool
        where T: Debug + PartialEq + PartialOrd + Clone
    {
        ys.iter()
            .fold((xs, true), |(xs, prev), y| {
                let head = xs.head();
                let tail = xs.tail();
                (tail, prev && &head == y)
            })
            .1
    }
}

