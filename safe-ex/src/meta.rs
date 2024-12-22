// Copyright (c) 2024 riveroon
// This file is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//const HASH_BITS: usize = 7;
const HASH_MASK: i8 = 0b0111_1111u8 as i8;

pub const OCCUPIED: i8 = 0b0000_0000u8 as i8;
pub const VACANT: i8 = 0b1000_0000u8 as i8;
pub const DELETED: i8 = 0b1111_1111u8 as i8;

pub type Meta = i8;

pub const fn occupied(hash: u64) -> i8 {
    (hash as i8 & HASH_MASK) | OCCUPIED
}
