#![forbid(unsafe_code)]

pub(crate) mod alloc;
pub(crate) mod meta;
pub(crate) mod prelude;
use prelude::*;

mod drain;
pub use drain::Drain;

use std::{borrow::Borrow, hash::{BuildHasher, Hash, RandomState}, mem};

pub struct HashMap<K, V, S = RandomState> {
    alloc: Alloc<(K, V)>,
    len: usize,
    deleted: usize,
    hasher: S
}

impl<K, V, S: Default> HashMap<K, V, S> {
    /// Creates an empty `HashMap``.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty `HashMap` with at least the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty `HashMap` with the given hasher.
    #[inline]
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates an empty `HashMap` with the given hasher and at least the specified capacity.
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            alloc: Alloc::new(capacity),
            len: 0,
            deleted: 0,
            hasher
        }
    }

    /// Returns the number of entries stored in this map.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of entries this map is able to store.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.alloc.size()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        let _ = self.drain();
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        let drain = Drain {
            drain: self.alloc.drain(),
            len: self.len,
        };

        self.len = 0;
        self.deleted = 0;

        drain
    }
}

impl<K, V, S> HashMap<K, V, S>
where 
    K: Hash + Eq,
    S: BuildHasher
{
    #[inline]
    fn resize(&mut self, size: usize) {
        assert!(self.len() <= size);

        let size = size.next_power_of_two();

        let mut alloc: Alloc<(K, V)> = Alloc::new(size);

        for (key, value) in self.alloc.drain() {
            let hash = self.hasher.hash_one(&key);

            let finder = finder::Insertable;
            let controller = controller::Count(alloc.size());

            let idx = alloc.find_ref(hash, finder, controller) 
                .supply(&alloc)
                .nth(0)
                .unwrap()
                .index();

            alloc.get_meta(idx).occupy(hash);
            alloc.buckets.insert(idx, (key, value));
        }

        self.alloc = alloc;
    }

    /// Reserves capacity for at least `additional` more entries.
    /// 
    /// # Panics
    /// 
    /// Panics if the new allocation size overflows [`usize`].
    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        let size = self.len() + additional;

        if size <= self.alloc.size() {
            return;
        }
        
        self.resize(size.next_power_of_two());
    }

    /// Shrinks the capacity of this map as much as possible.
    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.resize(self.len());
    }

    #[inline(always)]
    fn auto_reserve(&mut self) {
        if self.len + self.deleted + 1 > (self.capacity() / 8) * 7 {
            if self.len < (self.capacity() / 2) {
                self.resize(self.capacity());
            } else {
                self.reserve(8);
            }
        }
    }

    #[inline(always)]
    fn auto_shrink(&mut self) {
        if self.len < self.capacity() / 8 {
            self.resize(self.capacity() / 2);
        }
    }

    /// Inserts a key-value pair into this map.
    /// Returns a `Some(value)` if a value was present with the matching `key`.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = self.hasher.hash_one(&key);

        self.auto_reserve();

        let meta = meta::occupied(hash);
        let finder = finder::Match { meta };
        let controller = controller::Vacancy;

        match self.alloc.find_ref(hash, finder, controller)
            .supply(&self.alloc)
            .find(|entry| &entry.bucket().unwrap().0 == &key)
            .map(|entry| entry.index())
        {
            Some(idx) => {
                let v = &mut self.alloc.buckets[idx].1;
                return Some(mem::replace(v, value))
            },
            None => ()
        };

        let finder = finder::Insertable;
        let controller = controller::None;
        let idx = self.alloc.find_ref(hash, finder, controller)
            .supply(&self.alloc)
            .nth(0)
            .unwrap()
            .index();

        self.alloc.get_meta(idx).occupy(hash);
        self.alloc.buckets.insert(idx, (key, value));
        self.len += 1;

        None
    }

    /// Removes a value whose key matches the given `key` from this map.
    /// Returns `None` if the key was not present in the map.
    #[inline(always)]
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
    #[inline]
    pub fn remove_entry<Q> (&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hasher.hash_one(&key);

        let meta = meta::occupied(hash);
        let finder = finder::Match { meta };
        let controller = controller::Either(
            controller::Count(self.alloc.size()),
            controller::Vacancy
        );

        let idx = self.alloc.find_ref(hash, finder, controller)
            .supply(&self.alloc)
            .find(|entry| entry.bucket().unwrap().0.borrow() == key)
            .map(|entry| entry.index())?;

        self.alloc.get_meta(idx)
            .write(meta::DELETED);
        let entry = self.alloc.buckets.remove(idx);

        self.len -= 1;
        self.deleted += 1;

        self.auto_shrink();

        entry
    }

    /// Performs a lookup for a key, and returns a reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    #[inline]
    pub fn get<Q> (&mut self, key: &Q) -> Option<&V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hasher.hash_one(&key);

        let meta = meta::occupied(hash);
        let finder = finder::Match { meta };
        let controller = controller::Either(
            controller::Count(self.alloc.size()),
            controller::Vacancy
        );

        self.alloc.find(hash, finder, controller)
            .map(|(_, bucket)| bucket.unwrap())
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Performs a lookup for a key, and returns a mutable reference to the value if it exists.
    /// Returns `None` if the key was not present in the map.
    #[inline]
    pub fn get_mut<Q> (&mut self, key: &Q) -> Option<&mut V>
    where 
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>
    {
        let hash = self.hasher.hash_one(&key);

        let meta = meta::occupied(hash);
        let finder = finder::Match { meta };
        let controller = controller::Either(
            controller::Count(self.alloc.size()),
            controller::Vacancy
        );

        let idx = self.alloc.find_ref(hash, finder, controller)
            .supply(&self.alloc)
            .find(|entry| entry.bucket().unwrap().0.borrow() == key)?
            .index();

        self.alloc.buckets.get_mut(idx)
            .map(|(_, v)| v)
    }
}

impl<K, V, S> Drop for HashMap<K, V, S> {
    fn drop(&mut self) {
        self.clear();
    }
}