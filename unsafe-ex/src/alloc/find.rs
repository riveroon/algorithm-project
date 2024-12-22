use std::ops;

use crate::{meta, prelude::*};

type CountedController<C> = controller::Either<controller::Count, C>;

struct FindInner<F, C> {
    index: usize,
    finder: F,
    found: u32,
    controller: CountedController<C>,
    finished: bool
}

impl<F, C> FindInner<F, C> {
    fn new<T> (alloc: &Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let finished = alloc.size == 0;
        let controller = controller::Either(
            controller::Count(alloc.size()),
            controller
        );

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

enum MetaMutInner<'a> {
    Normal(&'a mut Meta),
    WithTrailing(&'a mut Meta, &'a mut Meta)
}

pub struct MetaMut<'a> {
    inner: MetaMutInner<'a>
}

impl MetaMut<'_> {
    fn new<T> (alloc: &mut Alloc<T>, idx: usize) -> Self {
        use MetaMutInner::*;

        let inner = match idx {
            ..GROUP_SIZE => unsafe { WithTrailing(
                &mut *alloc.meta.add(idx),
                &mut *alloc.meta.add(alloc.size() + idx),
            ) },
            _ => Normal( unsafe { &mut *alloc.meta.add(idx) } ),
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
            Meta::occupied(
                meta::Hash::new(hash)
            )
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
    alloc: &'a mut Alloc<T>,
    inner: FindInner<F, C>
}

impl<'a, T, F, C> FindMut<'a, T, F, C> {
    pub fn new(alloc: &'a mut Alloc<T>, index: usize, finder: F, controller: C) -> Self {
        let inner = FindInner::new(alloc, index, finder, controller);

        Self { alloc, inner }
    }

    pub fn alloc(&mut self) -> &mut Alloc<T> {
        &mut self.alloc
    }

    pub fn into_inner(self) -> &'a mut Alloc<T> {
        let Self { alloc, .. } = self;

        alloc
    }
}

impl<'a, T, F: Finder, C: Controller> Iterator for FindMut<'a, T, F, C> {
    type Item = (MetaMut<'a>, &'a mut MaybeUninit<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(self.alloc)
            .map(|i| (
                MetaMut::new(self.alloc, i),
                unsafe { &mut *self.alloc.buckets.add(i) }
            ))
    }
}