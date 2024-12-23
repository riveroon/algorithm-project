use std::iter::FusedIterator;
use std::vec;

/// An iterator that drains elements from the `HashMap`.
pub struct Drain<'a, K, V> {
    // We hold a drain of a `Vec<Option<(K, V)>>`.
    pub(crate) iter: vec::Drain<'a, Option<(K, V)>>,
    pub(crate) remaining: usize,
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Some(pair) = slot {
                // Found a real (K, V) entry
                self.remaining -= 1;
                return Some(pair);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K, V> ExactSizeIterator for Drain<'a, K, V> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a, K, V> FusedIterator for Drain<'a, K, V> {}

impl<'a, K, V> Drop for Drain<'a, K, V> {
    fn drop(&mut self) {
        // Exhaust the iterator so we truly "drain" everything
        for _ in self {}
    }
}
