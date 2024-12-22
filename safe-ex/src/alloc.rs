pub mod finder;
use find::MetaMut;
pub use finder::Finder;

pub mod find;
pub use find::{Find, FindRef};

pub mod controller;
pub use controller::Controller;
use drain::Drain;
use stable_vec::StableVec;

pub mod drain;

use crate::prelude::*;

pub(crate) const GROUP_SIZE: usize = 32;

pub(crate) struct Alloc<T> {
    pub meta: Box<[Meta]>,
    pub buckets: StableVec<T>,
}

impl<T> Alloc<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            return Self {
                meta: Box::new([]),
                buckets: StableVec::new(),
            };
        }

        let size = size.next_power_of_two();

        Self {
            meta: vec![meta::VACANT; size + GROUP_SIZE].into(),
            buckets: StableVec::with_capacity(size)
        }
    }

    pub fn size(&self) -> usize {
        self.buckets.capacity()
    }

    pub fn clear(&mut self) {
        self.meta
            .fill(meta::VACANT);
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain::new(self)
    }

    pub fn get_meta(&mut self, idx: usize) -> MetaMut {
        MetaMut::new(&mut self.meta, idx)
    }

    pub fn find<F: Finder, C:  Controller> (&self, hash: u64, finder: F, controller: C) -> Find<'_, T, F, C> {
        let index = hash as usize & self.size().saturating_sub(1);

        Find::new(self, index, finder, controller)
    }

    pub fn find_ref<F: Finder,  C: Controller> (&self, hash: u64, finder: F, controller: C) -> FindRef<T, F, C> {
        let index = hash as usize & self.size().saturating_sub(1);

        FindRef::new(self, index, finder, controller)
    }
}