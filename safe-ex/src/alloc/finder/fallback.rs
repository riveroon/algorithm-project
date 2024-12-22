use crate::prelude::*;

pub struct Match {
    pub meta: Meta,
}

impl Finder for Match {
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        // Best-effor basis for autovectorization
        let mut found = 0;
        for (i, &m) in group.iter().enumerate() {
            if m == self.meta {
                found |= 0b1 << i;
            }
        }

        found
    }
}

pub struct Occupied;

impl Finder for Occupied {
    fn find(&mut self, group: &[Meta; 32]) -> u32 {
        let mut found = 0;
        for (i, &m) in group.iter().enumerate() {
            if m.is_occupied() {
                found |= 0b1 << i;
            }
        }

        found
    }
}