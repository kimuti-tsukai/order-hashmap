use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
    mem,
};

mod iter;

use iter::Iter;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct OrdValue<K, V> {
    value: V,
    after_key: Option<K>,
    before_key: Option<K>,
}

impl<K, V> OrdValue<K, V> {
    fn with_before(value: V, before_key: Option<K>) -> Self {
        Self {
            value,
            after_key: None,
            before_key,
        }
    }

    fn set_after(&mut self, key: Option<K>) {
        self.after_key = key;
    }

    fn set_before(&mut self, key: Option<K>) {
        self.before_key = key;
    }

    fn change(&mut self, mut value: V) -> V {
        mem::swap(&mut self.value, &mut value);

        value
    }
}

#[derive(Clone, Default)]
pub struct OrdHashMap<K, V, S = RandomState> {
    map: HashMap<K, OrdValue<K, V>, S>,
    before_key: Option<K>,
    first_key: Option<K>,
    last_key: Option<K>,
}

impl<K, V> OrdHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            before_key: None,
            first_key: None,
            last_key: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            before_key: None,
            first_key: None,
            last_key: None,
        }
    }
}

impl<K, V, S> OrdHashMap<K, V, S> {
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> OrdHashMap<K, V, S> {
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        Some(&self.map.get(key)?.value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        Some(&mut self.map.get_mut(key)?.value)
    }

    pub fn iter(&self) -> Iter<'_, K, V, S> {
        Iter::from(self)
    }
}

impl<K: Eq + Hash + Clone, V, S: BuildHasher> OrdHashMap<K, V, S> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(v) = self.map.get_mut(&key) {
            Some(v.change(value))
        } else {
            if self.map.is_empty() {
                self.first_key = Some(key.clone());
            }

            self.last_key = Some(key.clone());

            self.map.insert(
                key.clone(),
                OrdValue::with_before(value, self.before_key.clone()),
            );

            if let Some(before_key) = &self.before_key {
                self.map.get_mut(before_key).unwrap().set_after(Some(key.clone()));
            }

            self.before_key = Some(key);

            None
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone, S: BuildHasher> OrdHashMap<K, V, S> {
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let OrdValue {
            value,
            after_key,
            before_key,
        } = self.map.get(key)?.clone();

        if let Some(ref after_key) = after_key {
            if let Some(after_value) = self.map.get_mut(after_key.borrow()) {
                after_value.set_before(before_key.clone());
            }
        } else {
            self.first_key = after_key.clone();
        }

        if let Some(ref before_key) = before_key {
            if let Some(before_value) = self.map.get_mut(before_key.borrow()) {
                before_value.set_after(after_key);
            }
        } else {
            self.last_key = before_key;
        }

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn insert() {
        let mut map = OrdHashMap::new();

        map.insert(10, String::from("10"));

        map.insert(5, String::from("5"));

        assert_eq!(map.get(&10), Some(&String::from("10")));

        assert_eq!(map.get(&0), None);
    }
}
