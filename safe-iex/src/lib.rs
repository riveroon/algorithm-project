mod drain;
pub use drain::Drain;

use std::{borrow::Borrow, hash::{BuildHasher, Hash, RandomState}};

#[derive(Clone)]
pub struct HashMap<K, V, S = RandomState> {
    // TODO: replace with struct members
    phantom: std::marker::PhantomData<(K, V, S)>
}

impl<K, V, S: Default> HashMap<K, V, S> {
    /// Creates an empty `HashMap``.
    pub fn new() -> Self {
        todo!()
    }

    /// Creates an empty `HashMap` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty `HashMap` with the given hasher.
    pub fn with_hasher(hasher: S) -> Self {
        todo!()
    }

    /// Creates an empty `HashMap` with the given hasher and at least the specified capacity.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        todo!()
    }

    /// Reserves capacity for at least `additional` more entries.
    /// 
    /// # Panics
    /// 
    /// Panics if the new allocation size overflows [`usize`].
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }

    /// Returns the number of entries stored in this map.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns the number of entries this map is able to store.
    pub fn capacity(&self) -> usize {
        todo!()
    }

    /// Shrinks the capacity of this map as much as possible.
    pub fn shrink_to_fit(&mut self) {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        let _ = self.drain();
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        todo!()
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
        todo!()
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
        todo!()
    }

    /// Performs a lookup for a key, and returns a reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    pub fn get<Q> (&mut self, key: &Q) -> Option<&V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        todo!()
    }

    /// Performs a lookup for a key, and returns a mutable reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    pub fn get_mut<Q> (&mut self, key: &mut Q) -> Option<&mut V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        todo!()
    }
}