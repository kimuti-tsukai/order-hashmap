use std::hash::{BuildHasher, Hash, RandomState};

use crate::{OrdHashMap, OrdValue};

#[derive(Clone)]
pub struct Iter<'a, K, V, S = RandomState> {
    map: &'a OrdHashMap<K, V, S>,
    now_key: Option<&'a K>,
    is_first: bool,
}

impl<'a, K, V, S> Iter<'a, K, V, S> {
    pub(crate) fn from(map: &'a OrdHashMap<K, V, S>) -> Self {
        Self {
            map,
            now_key: None,
            is_first: true,
        }
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> Iterator for Iter<'a, K, V, S> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let key = if self.is_first {
            self.is_first = false;
            self.map.first_key.as_ref()?
        } else {
            self.now_key?
        };

        let OrdValue {
            value,
            after_key,
            before_key: _,
        } = self.map.map.get(key)?;

        self.now_key = after_key.as_ref().to_owned();

        Some((key, value))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn iter() {
        let mut map = OrdHashMap::new();

        map.insert(10, String::from("10"));

        map.insert(5, String::from("5"));

        map.insert(100, String::from("This is 100"));

        let mut i = map.iter();

        assert_eq!(i.next(), Some((&10, &String::from("10"))));

        assert_eq!(i.next(), Some((&5, &String::from("5"))));

        assert_eq!(i.next(), Some((&100, &String::from("This is 100"))));

        assert_eq!(i.next(), None);
    }
}
