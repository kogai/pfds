use std::hash::Hash;
use std::collections::HashMap;
use set::FiniteMap;

#[derive(Debug, Clone, PartialEq)]
struct FiniteMapImpl<K: Eq + Hash + Clone, V: Clone> {
    table: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> FiniteMap<K, V> for FiniteMapImpl<K, V> {
    fn empty() -> Self {
        FiniteMapImpl { table: HashMap::new() }
    }

    fn bind(&self, key: K, value: V) -> Self {
        let mut table = self.table.clone();
        match table.insert(key, value) {
            Some(_) => self.clone(),
            None => FiniteMapImpl { table: table },
        }
    }

    fn lookup(&self, key: K) -> Option<V> {
        self.table.get(&key).map(|x| x.clone())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_bind() {
        let actual = FiniteMapImpl::empty().bind("key", 100);
        let mut table = HashMap::new();
        table.insert("key", 100);
        let expect = FiniteMapImpl { table: table };

        assert!(actual == expect);
        assert!(actual.bind("key", 100) == expect)
    }

    #[test]
    fn test_lookup() {
        let actual = FiniteMapImpl::empty().bind("key", 100);
        assert!(actual.lookup("key") == Some(100));
        assert!(actual.lookup("not-exist") == None);
    }

}
