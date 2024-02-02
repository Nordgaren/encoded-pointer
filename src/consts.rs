use std::mem::size_of;

pub const PTR_SIZE_IN_BITS: usize = size_of::<usize>() * 8;
pub const LAST_BIT: usize = PTR_SIZE_IN_BITS - 1;
pub const SECOND_TO_LAST_BIT: usize = PTR_SIZE_IN_BITS - 2;