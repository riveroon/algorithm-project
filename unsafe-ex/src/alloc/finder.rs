mod avx;
mod fallback;

use crate::prelude::*;

pub trait Finder {
    fn find(&mut self, meta: &[Meta; 32]) -> u32;
}


pub struct Any;

impl Finder for Any {
    #[inline(always)]
    fn find(&mut self, _: &[Meta; 32]) -> u32 {
        u32::MAX
    }
}

pub struct Either<T, U> (pub T, pub U);

impl<T, U> Finder for Either<T, U>
where
    T: Finder,
    U: Finder
{
    #[inline(always)]
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        self.0.find(group) | self.1.find(group)
    }
}

#[cfg(target_feature = "avx")]
pub use avx::*;
#[cfg(not(target_feature = "avx"))]
pub use fallback::*;

pub struct Insertable;

impl Finder for Insertable {
    #[inline(always)]
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        let mut occupied = Occupied {};
        !occupied.find(group)
    }
}