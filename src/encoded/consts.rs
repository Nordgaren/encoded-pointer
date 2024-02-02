use std::mem::size_of;

pub const PTR_SIZE_IN_BITS: usize = size_of::<usize>() * 8;
pub const BOOL_ONE_POSITION: usize = PTR_SIZE_IN_BITS - 1;
pub const BOOL_TWO_POSITION: usize = PTR_SIZE_IN_BITS - 2;
pub const BOOL_ONE_MASK: usize = 1 << BOOL_ONE_POSITION;
pub const BOOL_TWO_MASK: usize = 1 << BOOL_TWO_POSITION;
pub const ENCODED_MASK: usize = 0b11 << BOOL_TWO_POSITION;
