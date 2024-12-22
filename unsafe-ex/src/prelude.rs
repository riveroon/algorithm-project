pub(crate) use crate::alloc::*;
pub(crate) use crate::meta::Meta;

pub use std::mem::MaybeUninit;

#[inline(always)]
pub(crate) fn likely(b: bool) -> bool {
    !unlikely(!b)
}

#[inline(always)]
pub(crate) fn unlikely(b: bool) -> bool {
    #[cold]
    fn cold() {}

    if b { cold() }
    b
}