use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Vacancy;

impl Controller for Vacancy {
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        use std::arch::x86_64::{self, __m256i};

        let finished = unsafe {
            let group = x86_64::_mm256_loadu_si256(
                group as *const [Meta] as *const __m256i
            );

            let vacant = x86_64::_mm256_set1_epi8(u8::from(Meta::VACANT) as i8);

            let eq = x86_64::_mm256_cmpeq_epi8(group, vacant);

            x86_64::_mm256_testz_si256(eq, eq) == 0
        };

        finished
    }
}