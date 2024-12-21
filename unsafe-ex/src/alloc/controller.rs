mod avx;
mod fallback;

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
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        self.0 = self.0.saturating_sub(group.len());

        self.0 < GROUP_SIZE
    }
}

#[cfg(target_feature = "avx")]
pub use avx::*;
#[cfg(not(target_feature = "avx"))]
pub use fallback::*;

impl Default for Vacancy {
    fn default() -> Self {
        Self
    }
}

pub struct None;

impl Controller for None {
    fn finished(&mut self, _: &[Meta; 32]) -> bool {
        false
    }
}