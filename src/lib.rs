use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
    mem,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct OrdValue<K, V> {
    pub value: V,
    after_key: Option<K>,
}

impl<K, V> OrdValue<K, V> {
    fn new(value: V) -> Self {
        Self {
            value,
            after_key: None,
        }
    }

    fn set_after(&mut self, key: K) {
        self.after_key = Some(key);
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
}

impl<K, V> OrdHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            before_key: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            before_key: None,
        }
    }
}

impl<K, V, S> OrdHashMap<K, V, S> {
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> OrdHashMap<K, V, S> {
    pub fn get(&self, key: &K) -> Option<&V> {
        Some(&self.map.get(key)?.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Some(&mut self.map.get_mut(key)?.value)
    }
}

impl<K: Eq + Hash + Clone, V> OrdHashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(v) = self.map.get_mut(&key) {
            Some(v.change(value))
        } else {
            self.map.insert(key.clone(), OrdValue::new(value));

            if let Some(before_key) = &self.before_key {
                self.map.get_mut(before_key).unwrap().set_after(key);
            }

            None
        }
    }
}
