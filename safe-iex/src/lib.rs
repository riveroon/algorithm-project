mod drain;
pub use drain::Drain;

use std::{
    borrow::Borrow,
    hash::{BuildHasher, Hash, Hasher, RandomState},
};

pub struct HashMap<K, V, S = RandomState> {
    hasher: S,
    table: Vec<Option<(K, V)>>,
    items: usize, // number of occupied slots
    mask: usize,  // == table.len() - 1, for quick modulus
}

impl<K, V, S: Default> HashMap<K, V, S> {
    
    /// Creates an empty HashMap.
    pub fn new() -> Self {
        Self::with_capacity_and_hasher(0, S::default())
    }

    /// Creates an empty HashMap with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// Creates an empty HashMap with the given hasher.
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates an empty HashMap with the given hasher and at least the specified capacity.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        let capacity = capacity.next_power_of_two().max(8);
        let mut table = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            table.push(None);
        }
        let mask = capacity - 1;

        Self {
            hasher,
            table,
            items: 0,
            mask,
        }
    }


    pub fn len(&self) -> usize {
        self.items
    }

    pub fn capacity(&self) -> usize {
        self.table.len()
    }


    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        // For each slot in the table, set it to None:
        for slot in &mut self.table {
            *slot = None;
        }
        // Now we have 0 items, but the capacity remains the same.
        self.items = 0;
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        let iter = self.table.drain(..);
        let remaining = self.items;
        self.items = 0; 
        Drain { iter, remaining }
    }

}

impl<K, V, S> HashMap<K, V, S>
where K: Hash + Eq, S: BuildHasher
{
    pub fn shrink_to_fit(&mut self) {
        let minimal = self.items.next_power_of_two().max(8);
        if minimal < self.capacity() {
            self.resize(minimal);
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        let needed = self.items.saturating_add(additional);
        if needed > self.capacity() {
            let mut new_capacity = self.capacity();
            while needed * 2 > new_capacity {
                new_capacity *= 2;
            }
            self.resize(new_capacity);
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.items * 2 >= self.table.len() {
            self.resize(self.table.len() * 2);
        }

        let hash = self.make_hash(&key);
        let mut index = hash & self.mask;
        let mut probe = 0;
        loop {
            match &mut self.table[index] {
                Some((ref existing_key, ref mut existing_val)) => {
                    if existing_key == &key {
                        let old_val = std::mem::replace(existing_val, value);
                        return Some(old_val);
                    }
                }
                None => {
                    self.table[index] = Some((key, value));
                    self.items += 1;
                    return None;
                }
            }
            probe += 1;
            index = (hash + probe) & self.mask;

            if probe >= self.table.len() {
                panic!("HashMap full even after resize—shouldn't happen!");
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let pos = self.find_position(key)?;
        let pair = self.table[pos].take();
        if pair.is_some() {
            self.items -= 1;
            self.rehash(pos + 1);
        }
        pair
    }

    pub fn resize(&mut self, new_capacity: usize) {
        let new_capacity = new_capacity.next_power_of_two().max(8);

        let mut new_table = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_table.push(None);
        }

        let old_table = std::mem::replace(&mut self.table, new_table);
        self.mask = new_capacity - 1;
        self.items = 0;

        for slot in old_table.into_iter() {
            if let Some((k, v)) = slot {
                self.insert(k, v);
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let hash = self.make_hash(key);
        let mut index = hash & self.mask;
        let mut probe = 0;
        loop {
            match &self.table[index] {
                Some((ref k, ref v)) => {
                    if k.borrow() == key {
                        return Some(v);
                    }
                }
                None => {
                    return None;
                }
            }
            probe += 1;
            index = (hash + probe) & self.mask;
            if probe >= self.table.len() {
                return None;
            }
        }
    }

    /// NEW get_mut IMPLEMENTATION
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        // 1) find the slot index by reading the table immutably
        let pos = self.find_position(key)?;
        // 2) now do exactly one mutable borrow of that slot
        self.table[pos].as_mut().map(|(_, v)| v)
    }

    fn find_position<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let hash = self.make_hash(key);
        let mut index = hash & self.mask;
        let mut probe = 0;
        loop {
            match &self.table[index] {
                Some((ref k, _v)) => {
                    if k.borrow() == key {
                        return Some(index);
                    }
                }
                None => {
                    return None;
                }
            }
            probe += 1;
            index = (hash + probe) & self.mask;
            if probe >= self.table.len() {
                return None;
            }
        }
    }

    fn make_hash<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut state = self.hasher.build_hasher();
        key.hash(&mut state);
        state.finish() as usize
    }


    fn rehash(&mut self, from: usize) {
        let capacity = self.table.len();

        let old_table = match self.table[from..].iter().position(Option::is_none) {
            Some(len) => {
                let mut empty: Vec<Option<(K, V)>> = (0..len).map(|_| None).collect();
                self.table[from..from + len].swap_with_slice(&mut empty);
                empty
            },
            None => {
                let Some(len1) = self.table[0..].iter().position(Option::is_none) else { return };

                let len0 = self.table.len() - from;
                let mut vec: Vec<Option<(K, V)>> = (0..len0 + len1).map(|_| None).collect();
                self.table[from..].swap_with_slice(&mut vec[..len0]);
                
                self.table[0..len1].swap_with_slice(&mut vec[len0..]);

                vec
            }
        };

        self.items -= old_table.len();

        for slot in old_table.into_iter() {
            if let Some((k, v)) = slot {
                self.insert(k, v);
            }
        }
    }
}
