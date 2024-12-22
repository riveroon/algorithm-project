// Copyright (c) 2024 riveroon
// This file is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

//const HASH_BITS: usize = 7;
const HASH_MASK: u8 = 0b0111_1111;

const OCCUPIED: u8 = 0b0000_0000;
const VACANT: u8 = 0b1000_0000;
const DELETED: u8 = 0b1111_1111;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Hash {
    inner: u8
}

impl Hash {
    pub fn new(hash: u64) -> Self {
        Self {
            inner: hash as u8 & HASH_MASK
        }
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#07b}", self.inner)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Meta {
    inner: u8
}

impl Meta {
    pub const VACANT: Meta = Self {
        inner: VACANT
    };

    pub const DELETED: Meta = Self {
        inner: DELETED
    };

    pub const fn occupied(hash: Hash) -> Self {
        let inner = (hash.inner & HASH_MASK) | OCCUPIED as u8;
        
        Self { inner }
    }

    pub fn is_occupied(self) -> bool {
        (self.inner & !HASH_MASK) == 0
    }

    pub fn hash(self) -> Option<Hash> {
        match self.is_occupied() {
            true => Some ( Hash {
                inner: self.inner & HASH_MASK
            } ),
            false => None
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Meta::VACANT
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Meta::DELETED => write!(f, "Meta::Deleted"),
            Meta::VACANT => write!(f, "Meta::Vacant"),
            _ => write!(f, "Meta::Occupied({:?})", self.hash().unwrap())
        }
    }
}

impl From<Meta> for u8 {
    fn from(value: Meta) -> Self {
        value.inner
    }
}