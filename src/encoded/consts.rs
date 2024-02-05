/// The size of a pointer in bits.
pub const PTR_SIZE_IN_BITS: usize = usize::BITS as usize;
/// the bit position of the first encoded bool
pub const BOOL_ONE_POSITION: usize = PTR_SIZE_IN_BITS - 1;
/// the bit position of the second encoded bool
pub const BOOL_TWO_POSITION: usize = PTR_SIZE_IN_BITS - 2;
/// Mask with `BOOL_ONE_POSITION` set to 1
pub const BOOL_ONE_MASK: usize = 1 << BOOL_ONE_POSITION;
/// Mask with `BOOL_TWO_POSITION` set to 1
pub const BOOL_TWO_MASK: usize = 1 << BOOL_TWO_POSITION;
/// Mask that has the bits that are used for the encoded data set
pub const ENCODED_MASK: usize = 0b11 << BOOL_TWO_POSITION;
/// Mask that has the bits that are used for the address set
pub const ADDRESS_MASK: usize = !ENCODED_MASK;
