use crate::prelude::*;

pub struct Match {
    pub meta: Meta,
}

impl Finder for Match {
    #[inline]
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        use core::arch::x86_64::{self, __m256i};

        assert_eq!(GROUP_SIZE * 8, 256);

        unsafe {
            let group = x86_64::_mm256_loadu_si256(
                group as *const [Meta] as *const __m256i
            );
            
            let mask = x86_64::_mm256_set1_epi8(
                u8::from(self.meta) as i8
            );

            let eq = x86_64::_mm256_cmpeq_epi8(group, mask);

            x86_64::_mm256_movemask_epi8(eq) as u32
        }
    }
}

pub struct Occupied;

impl Finder for Occupied {
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        use core::arch::x86_64::{self, __m256i};

        assert_eq!(GROUP_SIZE * 8, 256);

        unsafe {
            let group = x86_64::_mm256_loadu_si256(
                group as *const [Meta] as *const __m256i
            );

            !x86_64::_mm256_movemask_epi8(group) as u32
        }
    }
}