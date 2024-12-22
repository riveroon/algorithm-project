use std::{iter::FusedIterator, mem::MaybeUninit};

use super::{controller, finder, Alloc, FindMut};

pub struct Drain<'a, T> {
    inner: FindMut<'a, T, finder::Occupied, controller::Count>
}

impl<'a, T> Drain<'a, T> {
    pub fn new(alloc: &'a mut Alloc<T>) -> Self {
        let finder = finder::Occupied;
        let controller = controller::Count(alloc.size);

        let inner = alloc.find_mut(0, finder, controller);

        Self { inner }
    }
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|(_, entry)| unsafe { entry.assume_init_read() })
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}

        let alloc = self.inner.alloc();
        if alloc.size() > 0 {
            unsafe { self.inner.alloc().clear() };
        }
    }
}