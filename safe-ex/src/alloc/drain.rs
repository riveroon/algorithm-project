use std::iter::FusedIterator;

use super::{controller, finder, Alloc, FindRef};

pub struct Drain<'a, T> {
    inner: FindRef<T, finder::Occupied, controller::Count>,
    alloc: &'a mut Alloc<T>
}

impl<'a, T> Drain<'a, T> {
    pub fn new(alloc: &'a mut Alloc<T>) -> Self {
        let finder = finder::Occupied;
        let controller = controller::Count(alloc.size());

        let inner = alloc.find_ref(0, finder, controller);

        Self { inner, alloc }
    }
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.inner.supply(&self.alloc)
            .next()?
            .index();
        
        self.alloc.buckets.remove(idx)
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}

        self.alloc.clear();
    }
}