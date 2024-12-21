mod drain;
pub use drain::Drain;

use std::collections::hash_map::RandomState;
use std::{borrow::Borrow, hash::{BuildHasher, Hash, Hasher}, mem};

#[derive(Clone)]
pub struct HashMap<K, V, S = RandomState> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
    hasher: S,
}

impl<K, V, S: Default + BuildHasher> HashMap<K, V, S> {
    /// Creates an empty `HashMap`.
    pub fn new() -> Self {
        Self::with_capacity_and_hasher(0, S::default())
    }

    /// Creates an empty `HashMap` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, V, S: BuildHasher> HashMap<K, V, S> {
    /// Hashes a key using the map's hasher.
    fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
        let mut hasher = self.hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Creates an empty `HashMap` with the given hasher and at least the specified capacity.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        let bucket_count = Self::bucket_count(capacity);
        Self { 
            buckets: vec![None; bucket_count],
            size: 0,
            hasher,
        }
    }

    /// Reserves capacity for at least `additional` more entries.
    pub fn reserve(&mut self, additional: usize) {
        let new_capacity = self.size + additional;
        if new_capacity > self.buckets.len() {
            self.resize(new_capacity);
        }
    }

    /// Returns the number of entries stored in this map.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns the number of entries this map is able to store.
    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    /// Shrinks the capacity of this map as much as possible.
    pub fn shrink_to_fit(&mut self) {
        let new_capacity = Self::bucket_count(self.size);
        if new_capacity < self.buckets.len() {
            self.resize(new_capacity);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.buckets.iter_mut().for_each(|slot| *slot = None);
        self.size = 0;
    }

    fn bucket_count(capacity: usize) -> usize {
        capacity.next_power_of_two().max(8)
    }

    fn resize(&mut self, new_capacity: usize) {
        let new_bucket_count = Self::bucket_count(new_capacity);
        if new_bucket_count == self.buckets.len() {
            return;
        }

        let mut new_buckets = vec![None; new_bucket_count];
        for slot in self.buckets.drain(..) {
            if let Some((key, value)) = slot {
                let hash = self.hash(&key);
                let mut index = hash % new_bucket_count;
                while new_buckets[index].is_some() {
                    index = (index + 1) % new_bucket_count;
                }
                new_buckets[index] = Some((key, value));
            }
        }
        self.buckets = new_buckets;
    }
}

impl<K, V, S> HashMap<K, V, S>
where 
    K: Hash + Eq + Clone,
    S: BuildHasher
{
    /// Inserts a key-value pair into this map.
    /// Returns a `Some(value)` if a value was present with the matching `key`.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= 3 * self.buckets.len() / 4 {
            self.resize(self.size * 2);
        }

        let hash = self.hash(&key);
        let mut index = hash % self.buckets.len();

        loop {
            match &mut self.buckets[index] {
                Some((existing_key, existing_value)) if *existing_key == key => {
                    return Some(mem::replace(existing_value, value));
                }
                None => {
                    self.buckets[index] = Some((key, value));
                    self.size += 1;
                    return None;
                }
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }
    }

    /// Removes a value whose key matches the given `key` from this map.
    /// Returns `None` if the key was not present in the map.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let mut index = hash % self.buckets.len();

        loop {
            match &mut self.buckets[index] {
                Some((existing_key, _)) if existing_key.borrow() == key => {
                    let slot = self.buckets[index].take();
                    self.size -= 1;

                    // Re-insert elements in the cluster
                    let mut next_index = (index + 1) % self.buckets.len();
                    while let Some((k, v)) = self.buckets[next_index].take() {
                        let rehash = self.hash(&k);
                        let mut target_index = rehash % self.buckets.len();
                        while self.buckets[target_index].is_some() {
                            target_index = (target_index + 1) % self.buckets.len();
                        }
                        self.buckets[target_index] = Some((k, v));
                        next_index = (next_index + 1) % self.buckets.len();
                    }

                    return slot.map(|(_, v)| v);
                }
                None => return None,
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }
    }

    /// Performs a lookup for a key, and returns a reference to the value if it exists.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let mut index = hash % self.buckets.len();

        loop {
            match &self.buckets[index] {
                Some((existing_key, value)) if existing_key.borrow() == key => return Some(value),
                None => return None,
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }
    }

    /// Performs a lookup for a key, and returns a mutable reference to the value if it exists.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let mut index = hash % self.buckets.len();

        loop {
            match &mut self.buckets[index] {
                Some((existing_key, value)) if existing_key.borrow() == key => return Some(value),
                None => return None,
                _ => {
                    index = (index + 1) % self.buckets.len();
                }
            }
        }
    }
}
