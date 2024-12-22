use wide::{i8x32, CmpEq};

use crate::prelude::*;

pub trait Controller {
    fn finished(&mut self, group: &[Meta; 32]) -> bool;
}


#[derive(Clone, Debug)]
pub struct Either<T, U> (pub T, pub U);

impl<T, U> Controller for Either<T, U>
where 
    T: Controller, 
    U: Controller
{
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        self.0.finished(group) || self.1.finished(group)
    }
}

#[derive(Clone, Debug)]
pub struct Count(pub usize);

impl Controller for Count {
    #[inline(always)]
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        self.0 = self.0.saturating_sub(group.len());

        self.0 < GROUP_SIZE
    }
}

#[derive(Clone, Debug)]
pub struct Vacancy;

impl Controller for Vacancy {
    #[inline(always)]
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        let finished = {
            let group = i8x32::from(&group[..]);

            let vacant = i8x32::from(meta::VACANT);

            let eq = group.cmp_eq(vacant);

            eq.move_mask() != 0
        };

        finished
    }
}
impl Default for Vacancy {
    fn default() -> Self {
        Self
    }
}

pub struct None;

impl Controller for None {
    #[inline(always)]
    fn finished(&mut self, _: &[Meta; 32]) -> bool {
        false
    }
}