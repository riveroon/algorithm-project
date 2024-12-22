use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Vacancy;

impl Controller for Vacancy {
    fn finished(&mut self, group: &[Meta; 32]) -> bool {
        let mut finished = false;
        for m in group {
            if *m == Meta::VACANT {
                finished = true;
            }
        }

        finished
    }
}