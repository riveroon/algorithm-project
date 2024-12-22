use std::iter::FusedIterator;

use super::{controller, finder, Alloc, FindMut};

pub struct Drain<'a, T> {
    inner: FindMut<'a, T, finder::Occupied, controller::None>
}

impl<'a, T> Drain<'a, T> {
    #[inline]
    pub fn new(alloc: &'a mut Alloc<T>) -> Self {
        let finder = finder::Occupied;
        let controller = controller::None;

        let inner = unsafe { alloc.find_mut(0, finder, controller) };

        Self { inner }
    }
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;
    
    #[inline]
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