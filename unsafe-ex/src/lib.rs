mod drain;
pub use drain::Drain;

use std::{borrow::Borrow, hash::{BuildHasher, Hash, RandomState}};

#[derive(Clone)]
pub struct HashMap<K, V, S = RandomState> {
    // TODO: replace with struct members
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
    hasher: S,
}

impl<K, V, S: Default> HashMap<K, V, S> {
    /// Creates an empty `HashMap``.
    pub fn new() -> Self {
        Self::with_capacity_and_hasher(0, S::default())
    }

    /// Creates an empty `HashMap` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
}
    
impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty `HashMap` with the given hasher.
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates an empty `HashMap` with the given hasher and at least the specified capacity.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self { 
            let bucket_count = Self::bucket_count(capacity);
            buckets: vec![Vec::new(); bucket_count],
            size: 0,
            hasher,
         }
    }

    /// Reserves capacity for at least `additional` more entries.
    /// 
    /// # Panics
    /// 
    /// Panics if the new allocation size overflows [`usize`].
    pub fn reserve(&mut self, additional: usize) {
        let new_capacity = self.size + additional;
        if new_capacity > self.buckets.len() {
            self.resize(new_capacity);
        }
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
            self.resize(self.size);
        }
        for bucket in &mut self.buckets {
            bucket.shrink_to_fit();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.buckets.clear ();
        self.size = 0;
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        self.size = 0;
        Drain {iter: self.buckets.drain(..)}
    }

    fn bucket_count(capacity: usize) -> usize {
        capacity.next_power_of_two().max(8)
    }

    fn resize(&mut self, new_capacity: usize) {
        let new_bucket_count = Self::bucket_count(new_capacity);
        if new_bucket_count == self.buckets.len() {
            return;
        }

        let mut new_buckets = vec![Vec::new(); new_bucket_count];
        for bucket in self.buckets.drain(..) {
            for (key, value) in bucket {
                let hash = self.hash(&key);
                let index = hash % new_bucket_count;
                new_buckets[index].push((key, value));
            }
        }
        self.buckets = new_buckets;
    }

}

impl<K, V, S> HashMap<K, V, S>
where 
    K: Hash + Eq,
    S: BuildHasher
{
    /// Inserts a key-value pair into this map.
    /// Returns a `Some(value)` if a value was present with the matching `key`.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= 3 * self.buckets.len() / 4 {
            self.resize(self.size * 2);
        }

        let hash = self.hash(&key);
        let bucket = &mut self.buckets[hash % self.buckets.len()];

        for pair in bucket.iter_mut() {
            if pair.0 == key {
                return Some(mem::replace(&mut pair.1, value));
            }
        }

        bucket.push((key, value));
        self.size += 1;
        None
    }

    /// Removes a value whose key matches the given `key` from this map.
    /// Returns `None` if the key was not present in the map.
    pub fn remove<Q> (&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        self.remove_entry(key)
            .map(|(_, v)| v)
    }

    /// Removes a key-value pair whose key matches the given `key` from this map.
    /// Returns `None` if the key was not present in the map.
    pub fn remove_entry<Q> (&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let bucket = &mut self.buckets[hash % self.buckets.len()];

        let index = bucket.iter().position(|(k, _)| k.borrow() == key)?;
        self.size -= 1;
        Some(bucket.swap_remove(index))
    }

    /// Performs a lookup for a key, and returns a reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    pub fn get<Q> (&mut self, key: &Q) -> Option<&V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let bucket = &self.buckets[hash % self.buckets.len()];

        bucket
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Performs a lookup for a key, and returns a mutable reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    pub fn get_mut<Q> (&mut self, key: &mut Q) -> Option<&mut V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hash(key);
        let bucket = &mut self.buckets[hash % self.buckets.len()];

        bucket
            .iter_mut()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }
}

pub struct Drain<'a, K, V> {
    iter: vec::Drain<'a, Vec<(K, V)>>,
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|mut bucket| bucket.pop())
    }
}