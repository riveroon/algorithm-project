use std::{mem::{self, MaybeUninit}, slice};

use crate::prelude::*;

struct FindInner<F, C> {
    index: usize,
    finder: F,
    found: u32,
    controller: C,
    finished: bool
}

impl<F, C> FindInner<F, C> {
    fn new<T> (alloc: &Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let finished = alloc.size == 0;

        Self {
            index,
            finder,
            found: 0,
            controller,
            finished
        }
    }
}

impl<F: Finder, C: Controller> FindInner<F, C> {
    fn next<T> (&mut self, alloc: &Alloc<T>) -> Option<usize> {
        loop {
            if self.found != 0 {
                let found = self.found.trailing_zeros() as usize;
                self.found ^= 0b1 << found;

                return Some((self.index + found) & (alloc.size - 1));
            }
    
            if self.finished {
                return None;
            }

            // SAFETY:
            //   1. All indexes of alloc.meta is a valid initialized Meta object.
            //   2. This Iterator has immutable access to Alloc.
            //   3. alloc.meta has a length of alloc.size + GROUP_SIZE - 1; therefore, we only need to ensure that self.index < alloc.size.
            let group = unsafe {
                &mut *(alloc.meta.add(self.index) as *mut _)
            };

            self.found = self.finder.find(group);

            self.finished = likely(self.controller.finished(group));

            self.index = (self.index + GROUP_SIZE) & (alloc.size - 1);
        }
    }
}

pub struct Find<'a, T, F, C> {
    alloc: &'a Alloc<T>,
    inner: FindInner<F, C>
}

impl<'a, T, F, C> Find<'a, T, F, C> {
    pub fn new(alloc: &'a Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let inner = FindInner::new(alloc, index, finder, controller);

        Self { alloc, inner }
    }
}

impl<'a, T, F: Finder, C: Controller> Iterator for Find<'a, T, F, C> {
    type Item = (&'a Meta, &'a MaybeUninit<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(self.alloc)
            .map(|i| unsafe {(
                &*self.alloc.meta.add(i),
                &*self.alloc.buckets.add(i)
            )})
    }
}

pub struct FindMut<'a, T, F, C> {
    alloc: mem::ManuallyDrop<&'a mut Alloc<T>>,
    inner: mem::ManuallyDrop<FindInner<F, C>>
}

impl<'a, T, F, C> FindMut<'a, T, F, C> {
    pub fn new(alloc: &'a mut Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let inner = FindInner::new(alloc, index, finder, controller);

        Self {
            alloc: mem::ManuallyDrop::new(alloc),
            inner: mem::ManuallyDrop::new(inner)
        }
    }

    pub fn alloc(&mut self) -> &mut Alloc<T> {
        &mut self.alloc
    }

    //TODO: Evaluate if this needs to be unsafe
    pub fn into_inner(mut self) -> &'a mut Alloc<T> {
        // SAFETY: self is forgotten below.
        let alloc = unsafe { mem::ManuallyDrop::take(&mut self.alloc) };
        let inner = unsafe { mem::ManuallyDrop::take(&mut self.inner) };
        
        alloc.trailing_meta();

        mem::forget(self);
        drop(inner);

        alloc
    }
}

impl<'a, T, F: Finder, C: Controller> Iterator for FindMut<'a, T, F, C> {
    type Item = (&'a mut Meta, &'a mut MaybeUninit<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(*self.alloc)
            .map(|i| unsafe {(
                &mut *self.alloc.meta.add(i),
                &mut *self.alloc.buckets.add(i)
            )})
    }
}

impl<T, F, C> Drop for FindMut<'_, T, F, C> {
    fn drop(&mut self) {
        self.alloc.trailing_meta();
    }
}