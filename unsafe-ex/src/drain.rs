use std::iter::FusedIterator;

pub struct Drain<'a, K, V> {
    // TODO: replace with struct members
    phantom: std::marker::PhantomData<&'a (K, V)>
}

impl<K, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();

        (len, Some(len))
    }
}

impl<K, V> ExactSizeIterator for Drain<'_, K, V> {
    fn len(&self) -> usize {
        todo!()
    }
}

impl<K, V> FusedIterator for Drain<'_, K, V> {}

impl<K, V> Drop for Drain<'_, K, V> {
    fn drop(&mut self) {
        for _ in self {}
    }
}