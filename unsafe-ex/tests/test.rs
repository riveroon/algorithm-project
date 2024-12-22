use std::{borrow::Borrow, ops::{Deref, DerefMut}};

pub use unsafe_ex::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Token(Box<usize>);

impl Token {
    pub fn new(value: usize) -> Self {
        Self(Box::new(value))
    }
}

impl Borrow<usize> for Token {
    fn borrow(&self) -> &usize {
        &self
    }
}

impl Deref for Token {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

#[test]
pub fn insert_lookup() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(!i));
    }

    assert_eq!(map.len(), 100);

    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&Token::new(!i)));
    }
}

#[test]
pub fn insert_replace() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(!i));
    }

    assert_eq!(map.len(), 100);

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(i));
    }

    assert_eq!(map.len(), 100);

    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&Token::new(i)));
    }
}

#[test]
pub fn remove() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(!i));
    }

    for i in 0..100 {
        assert_eq!(map.remove(&i), Some(Token::new(!i)));
        assert_eq!(map.get(&i), None);
        assert_eq!(map.len(), 99 - i);
    }

    assert!(map.is_empty());
}

#[test]
pub fn modify_lookup() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(!i));
    }

    for i in 0..100 {
        **map.get_mut(&i).unwrap() ^= 0b1;
    }

    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&Token::new(!i ^ 0b1)));
    }
}

#[test]
pub fn clear() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(!i));
    }

    assert_eq!(map.len(), 100);

    map.clear();

    assert_eq!(map.len(), 0);

    for i in 0..100 {
        assert_eq!(map.get(&i), None);
    }
}

pub fn drain() {
    let mut map: HashMap<Token, Token> = HashMap::new();

    for i in 0..100 {
        map.insert(Token::new(i), Token::new(i));
    }

    let mut returned = [false; 100];

    for (k, v) in map.drain() {
        assert_eq!(returned[*k], false);
        returned[*k] = true;
        assert_eq!(v, k)
    }

    assert_eq!(returned, [true; 100]);
    assert_eq!(map.len(), 0);
}