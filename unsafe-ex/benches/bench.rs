#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

#[bench]
pub fn insert_small(b: &mut Bencher) {
    todo!()
}

#[bench]
pub fn insert_remove_small(b: &mut Bencher) {
    todo!()
}

#[bench]
pub fn lookup_small(b: &mut Bencher) {
    todo!()
}

#[bench]
pub fn insert_big(b: &mut Bencher) {
    todo!()
}

#[bench]
pub fn insert_remove_big(b: &mut Bencher) {
    todo!()
}


#[bench]
pub fn lookup_big(b: &mut Bencher) {
    todo!()
}