pub use safe_iex::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Key(pub usize);

#[test]
pub fn insert_lookup() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), !i);
    }

    assert_eq!(map.len(), 100);

    for i in 0..100 {
        assert_eq!(map.get(&Key(i)), Some(&!i));
    }
}

#[test]
pub fn insert_replace() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), !i);
    }

    assert_eq!(map.len(), 100);

    for i in 0..100 {
        map.insert(Key(i), i);
    }
}

#[test]
pub fn remove() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), !i);
    }

    for i in 0..100 {
        let key = Key(i);

        assert_eq!(map.remove(&key), Some(!i));
        assert_eq!(map.get(&key), None);
        assert_eq!(map.len(), 99 - i);
    }

    assert!(map.is_empty());
}

#[test]
pub fn modify_lookup() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), !i);
    }

    for i in 0..100 {
        *map.get_mut(&Key(i)).unwrap() ^= 0b1;
    }

    for i in 0..100 {
        assert_eq!(map.get(&Key(i)), Some(&(!i ^ 0b1)));
    }
}

#[test]
pub fn clear() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), !i);
    }

    assert_eq!(map.len(), 100);

    map.clear();

    assert_eq!(map.len(), 0);

    for i in 0..100 {
        assert_eq!(map.get(&Key(i)), None);
    }
}

pub fn drain() {
    let mut map: HashMap<Key, usize> = HashMap::new();

    for i in 0..100 {
        map.insert(Key(i), i);
    }

    let mut returned = [false; 100];

    for (Key(k), v) in map.drain() {
        assert_eq!(returned[k], false);
        returned[k] = true;

        assert_eq!(v, k);
    }

    assert_eq!(returned, [true; 100]);
    assert_eq!(map.len(), 0);
}