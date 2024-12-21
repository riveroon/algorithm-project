use std::iter::FusedIterator;

use crate::alloc::drain;

pub struct Drain<'a, K, V> {
    pub(crate) drain: drain::Drain<'a, (K, V)>,
    pub(crate) len: usize
}

impl<K, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();

        (len, Some(len))
    }
}

impl<K, V> ExactSizeIterator for Drain<'_, K, V> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<K, V> FusedIterator for Drain<'_, K, V> {}