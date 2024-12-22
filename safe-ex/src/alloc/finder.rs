use wide::{i8x32, CmpEq};

use crate::prelude::*;

pub trait Finder {
    fn find(&mut self, meta: &[Meta; 32]) -> u32;
}


pub struct Any;

impl Finder for Any {
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
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        self.0.find(group) | self.1.find(group)
    }
}

pub struct Match {
    pub meta: Meta,
}

impl Finder for Match {
    #[inline]
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        assert_eq!(GROUP_SIZE * 8, 256);
        
        let group = i8x32::from(&group[..]);
        
        let mask = i8x32::splat(self.meta);

        group.cmp_eq(mask)
            .move_mask() as u32
    }
}

pub struct Occupied;

impl Finder for Occupied {
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        assert_eq!(GROUP_SIZE * 8, 256);

        !i8x32::from(&group[..])
            .move_mask() as u32
    }
}

pub struct Insertable;

impl Finder for Insertable {
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        let mut occupied = Occupied {};
        !occupied.find(group)
    }
}