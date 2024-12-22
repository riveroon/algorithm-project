pub mod finder;
pub use finder::Finder;

pub mod find;
pub use find::{Find, FindMut};

pub mod controller;
pub use controller::Controller;
use drain::Drain;

pub mod drain;

use std::{alloc, mem::MaybeUninit, ptr, slice};

use crate::Meta;

pub(crate) const GROUP_SIZE: usize = 32;

fn layout<T> (size: usize) -> (alloc::Layout, usize) {
    let meta = alloc::Layout::array::<Meta> (size + GROUP_SIZE).unwrap();
    let buckets = alloc::Layout::array::<T> (size).unwrap();

    meta.extend(buckets)
        .unwrap()
}

pub(crate) struct Alloc<T> {
    pub meta: *mut Meta,
    pub buckets: *mut MaybeUninit<T>,
    size: usize
}

impl<T> Alloc<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            return Self {
                meta: ptr::null_mut(),
                buckets: ptr::null_mut(),
                size: 0
            };
        }

        let size = size.next_power_of_two();

        let (layout, offset) = layout::<T> (size);
        let alloc = unsafe { alloc::alloc(layout) };

        let meta = alloc as *mut Meta;
        let buckets = unsafe { alloc.byte_add(offset) } as *mut MaybeUninit<T>;

        // SAFETY:
        //   1. All indexes of self.meta is a valid *MaybeUninit* object.
        //   2. The newly created metadata slice is never exposed.
        //
        // Set size to size + GROUP_SIZE instead of size + GROUP_SIZE - 1 for potentially more efficient memcpy.
        unsafe { slice::from_raw_parts_mut(meta, size + GROUP_SIZE) }
            .fill(Meta::VACANT);

        Self { meta, buckets, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// # SAFETY
    /// The caller must ensure that the called `Alloc` object does not have a size of 0.
    pub unsafe fn clear(&mut self) {
        slice::from_raw_parts_mut(self.meta, self.size + GROUP_SIZE)
            .fill(Meta::VACANT);
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain::new(self)
    }

    pub fn trailing_meta(&mut self) {
        unsafe {
            self.meta.copy_to_nonoverlapping(
                self.meta.add(self.size),
                GROUP_SIZE
            );
        }
    }

    pub fn find<F: Finder, C:  Controller> (&self, hash: u64, finder: F, controller: C) -> Find<'_, T, F, C> {
        let index = hash as usize & self.size.saturating_sub(1);

        Find::new(self, index, finder, controller)
    }

    pub fn find_mut<F: Finder,  C: Controller> (&mut self, hash: u64, finder: F, controller: C) -> FindMut<'_, T, F, C> {
        let index = hash as usize & self.size.saturating_sub(1);

        FindMut::new(self, index, finder, controller)
    }
}

impl<T> Drop for Alloc<T> {
    fn drop(&mut self) {
        let (layout, _) = layout::<T> (self.size);
        unsafe { alloc::dealloc(self.meta as *mut u8, layout) };
    }
}