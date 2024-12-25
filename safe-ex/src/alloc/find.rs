use std::{fmt::Debug, ops};

use crate::{meta, prelude::*};

type CountedController<C> = controller::Either<controller::Count, C>;

struct FindInner<F, C> {
    index: usize,
    finder: F,
    found: u32,
    controller: C,
    finished: bool
}

impl<F, C> FindInner<F, C> {
    fn new<T> (alloc: &Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let finished = alloc.size() == 0;

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

                return Some((self.index - GROUP_SIZE + found) & (alloc.size() - 1));
            }
    
            if self.finished {
                return None;
            }

            self.index &= alloc.size() - 1;

            let group = &alloc.meta[self.index..self.index + GROUP_SIZE].try_into().unwrap();

            let mask = !u32::MAX.checked_shl(alloc.size() as u32)
                .unwrap_or(0);
            self.found = self.finder.find(group) & mask;

            self.finished = likely(self.controller.finished(group));

            self.index = self.index + GROUP_SIZE;
        }
    }
}

#[derive(Debug)]
enum MetaMutInner<'a> {
    Normal(&'a mut Meta),
    WithTrailing(&'a mut Meta, &'a mut Meta)
}

pub struct MetaMut<'a> {
    inner: MetaMutInner<'a>
}

impl<'a> MetaMut<'a> {
    pub fn new(alloc: &'a mut [i8], idx: usize) -> Self {
        use MetaMutInner::*;

        let inner = match idx {
            ..GROUP_SIZE => {
                let (a, b) = alloc.split_at_mut(alloc.len() - GROUP_SIZE);
                WithTrailing(&mut a[idx], &mut b[idx])
            },
            _ => Normal( &mut alloc[idx] ),
        };

        Self { inner }
    }

    pub fn write(&mut self, meta: Meta) {
        use MetaMutInner::*;

        match &mut self.inner {
            Normal(m) => **m = meta,
            WithTrailing(m, trailing) => {
                **m = meta;
                **trailing = meta;
            }
        }
    }

    pub fn occupy(&mut self, hash: u64) {
        self.write(
            meta::occupied(hash)
        );
    }
}

impl ops::Deref for MetaMut<'_> {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        use MetaMutInner::*;

        match &self.inner {
            Normal(meta) => meta,
            WithTrailing(meta, _) => meta
        }
    }
}

impl Debug for MetaMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
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
    type Item = (&'a Meta, Option<&'a T>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(self.alloc)
            .map(|i| (
                &self.alloc.meta[i],
                self.alloc.buckets.get(i)
            ))
    }
}

#[derive(Clone, Copy)]
pub struct EntryRef<'a, T> {
    alloc: &'a Alloc<T>,
    idx: usize
}

impl<'a, T> EntryRef<'a, T> {
    pub fn meta(&self) -> &Meta {
        &self.alloc.meta[self.idx]
    }

    pub fn bucket(&self) -> Option<&T> {
        self.alloc.buckets.get(self.idx)
    }

    pub fn index(&self) -> usize {
        self.idx
    }
}

pub struct FindRef<T, F, C> {
    inner: FindInner<F, C>,
    _phantom: std::marker::PhantomData<T>
}

impl<T, F, C> FindRef<T, F, C> {
    pub fn new(alloc: &Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let inner = FindInner::new(alloc, index, finder, controller);

        Self { inner, _phantom: std::marker::PhantomData }
    }

    pub fn supply<'a> (&'a mut self, alloc: &'a Alloc<T>) -> FindRefIter<'a, T, F, C> {
        FindRefIter { inner: self, alloc }
    }
}

pub struct FindRefIter<'a, T, F, C> {
    inner: &'a mut FindRef<T, F, C>,
    alloc: &'a Alloc<T>
}

impl<'a, T, F: Finder, C: Controller> Iterator for FindRefIter<'a, T, F, C> {
    type Item = EntryRef<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.inner.next(self.alloc)
            .map(|idx| EntryRef { alloc: &self.alloc, idx })
    }
}

