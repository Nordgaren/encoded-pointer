pub(crate) mod consts;

use crate::decoded::{DecodedPointer, DecodedPointerMut};
use crate::encoded::consts::*;
use std::fmt::{Debug, Formatter};
use std::io::{Error, ErrorKind};
use std::mem::size_of;

/// This is a type that encodes the last two bits of a pointer with bools, to store extra data about a particular
/// pointer. It allows the user to get address portion of the type as a `usize` or a `*const/mut T`, either of the
/// encoded bools, as well as the entire encoded pointer. It also allows setting of the individual components.
///
/// The user can also choose to use the `DecodedPointer<T>` and `DecodedPointerMut<T>` structs to deconstruct the encoded
/// pointer to it's base parts.
///
/// This type also implements `Copy` and `Clone`, so can be passed by value without having issues with the borrow checker.
///
/// It also implements `From<EncodedPointer>` for `*mut T` and `*const T`, as well as `Debug` for convenience.
///
/// # Example
///
/// ```rust
/// # use encoded_pointer::encoded::EncodedPointer;
/// # use encoded_pointer::decoded::DecodedPointer;
/// # // These are just some hidden functions for the example that shows up in the documentation.
/// # fn check_some_value(pointer: *const u8) -> bool {
/// #   pointer as usize == 0
/// # }
/// # fn check_some_other_value(pointer: *const u8) -> bool {
/// #   pointer as usize != 0
/// # }
/// fn example(some_address: usize) {
///     let mut encoded = EncodedPointer::from_address(some_address)
///             .expect("Could not encode pointer");
///     let pointer = encoded.get_pointer();
///     let bool_one = check_some_value(pointer);
///     encoded.set_bool_one(bool_one);///
///
///     let bool_two = check_some_other_value(pointer);
///     encoded.set_bool_two(bool_two);
///
///     inner_example(&encoded);
/// }
///
/// fn inner_example(pointer: &EncodedPointer) {
///     let DecodedPointer {
///         pointer: buffer,
///         bool_one: something_i_care_about,
///         bool_two: something_else_i_care_about,
///     } = pointer.get_decoded::<u8>();
///
///     if something_i_care_about {
///         // Do something
///     }
///
///     if something_else_i_care_about {
///         // Do something
///     }
///
///     // Do something with the pointer
///     let byte = unsafe { buffer.read() };
/// }
/// ```
pub struct EncodedPointer {
    value: usize,
}
// Assert that the EncodedPointer is the same size as a usize.
const _: () = assert!(size_of::<EncodedPointer>() == size_of::<usize>());

impl EncodedPointer {
    /// Checks if there is bit collision in the provided pointer, and then returns an EncodedPointer with the given
    /// bool values encoded into the pointer. If there is bit collision, it returns Error with ErrorKind::InvalidInput.
    pub fn new(pointer: usize, bool_one: bool, bool_two: bool) -> std::io::Result<Self> {
        if bit_collision(pointer) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Pointer contains data in the two most significant bits, and can't be encoded.",
            ));
        }

        let value = Self::encode(pointer, bool_one, bool_two);
        Ok(EncodedPointer { value })
    }
    /// Checks if there is bit collision in the provided pointer, and then returns an EncodedPointer with the encoded
    /// bools set to false. If there is bit collision, it returns Error with ErrorKind::InvalidInput.
    pub fn from_address(address: usize) -> std::io::Result<Self> {
        Self::new(address, false, false)
    }
    /// Returns an encoded pointer with the provided value, without checking for bit collision. Assumes the given value
    /// is a valid EncodedPointer.
    ///
    /// ### Does not do any checking for bit collision.
    ///
    /// # Safety
    /// This function does not check if there is bit collision. It is up to the user to pass in a valid value that either
    /// contains none of the upper bits set, or the correct upper bits set that represent the values in the encoded pointer
    /// you are creating.
    pub unsafe fn from_value_unchecked(value: usize) -> EncodedPointer {
        EncodedPointer { value }
    }
    /// Returns a DecodedPointer<T> with the specified const pointer type.
    pub fn get_decoded<T>(&self) -> DecodedPointer<T> {
        self.into()
    }
    /// Returns a DecodedPointerMut<T> with the specified mutable pointer type.
    pub unsafe fn get_decoded_mut<T>(&mut self) -> DecodedPointerMut<T> {
        DecodedPointerMut {
            pointer: unsafe { self.get_pointer_mut() },
            bool_one: self.get_bool_one(),
            bool_two: self.get_bool_two(),
        }
    }
    /// Returns the entire encoded pointer value, including the encoded bools.
    ///
    /// # Safety
    ///
    /// The caller can use this value to "clone" the `EncodedPointer`. This could lead to undefined behaviour.
    /// Especially if the cloned `EncodedPointer` gets used as a mutable reference. Prefer to use a
    /// reference or some type of `Rc<EncodedPointer>` to the EncodedPointer, instead of cloning it.
    #[inline(always)]
    pub unsafe fn get_value(&self) -> usize {
        self.value
    }
    /// Returns the address portion of the encoded pointer.
    #[inline(always)]
    pub fn get_address(&self) -> usize {
        self.value & ADDRESS_MASK
    }
    /// Returns a const pointer to the generic type, using the address portion of the encoded pointer.
    #[inline(always)]
    pub fn get_pointer<T>(&self) -> *const T {
        self.get_address() as *const T
    }
    /// Returns a mutable pointer to the generic type, using the address portion of the encoded pointer.
    ///
    /// # Safety
    ///
    /// The caller must uphold Rusts mutability invariance, such that, they guarantee there is no other
    /// mutable reference to this data,besides the `EncodedPointer`. Mutating data that has another mutable
    /// reference could lead to undefined behaviour.
    #[inline(always)]
    pub unsafe fn get_pointer_mut<T>(&mut self) -> *mut T {
        self.get_address() as *mut T
    }
    /// Returns the bool encoded into the last bit of the pointer.
    #[inline(always)]
    pub fn get_bool_one(&self) -> bool {
        self.value & BOOL_ONE_MASK != 0
    }
    /// Returns the bool encoded into the second to last bit of the pointer.
    #[inline(always)]
    pub fn get_bool_two(&self) -> bool {
        self.value & BOOL_TWO_MASK != 0
    }
    /// Sets the entire encoded pointer to the given value.
    ///
    /// # Safety
    ///
    /// This function does not check if there is bit collision. It is up to the user to pass in a valid value that either
    /// contains none of the upper bits set, or the correct upper bits set that represent the correct values for the encoded pointer.
    #[inline(always)]
    pub unsafe fn set_value(&mut self, value: usize) {
        self.value = value
    }
    /// Checks if the address has bit collision with the encoded bool portion of the encoded pointer. Sets the _address portion of the encoded
    /// pointer to the address given, if there is no collision, and returns true. Returns false if the address could not
    /// be set.
    #[inline(always)]
    pub fn set_address(&mut self, pointer: usize) -> bool {
        if bit_collision(pointer) {
            return false;
        }
        let encoded = self.value & ENCODED_MASK;
        self.value = pointer | encoded;
        true
    }
    /// Sets the last bit of the encoded pointer to the provided value.
    pub fn set_bool_one(&mut self, b: bool) {
        self.value &= !BOOL_ONE_MASK;
        self.value |= (b as usize) << BOOL_ONE_POSITION;
    }
    /// Sets the second to last bit of the encoded pointer to the provided value.
    #[inline(always)]
    pub fn set_bool_two(&mut self, b: bool) {
        self.value &= !BOOL_TWO_MASK;
        self.value |= (b as usize) << BOOL_TWO_POSITION;
    }
}
impl EncodedPointer {
    /// Takes in a usize and two bools, and returns a usize with the two bools encoded into the last two bits of the usize.
    ///
    /// ### Does not do any checking for bit collision.
    #[inline(always)]
    pub fn encode(pointer: usize, bool_one: bool, bool_two: bool) -> usize {
        pointer
            | ((bool_one as usize) << BOOL_ONE_POSITION)
            | ((bool_two as usize) << BOOL_TWO_POSITION)
    }
}
/// Returns true if any of the bits used for encoding are set.
fn bit_collision(pointer: usize) -> bool {
    pointer & ENCODED_MASK != 0
}

impl Debug for EncodedPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : 0x{:X}", self.get_decoded::<u8>(), self.value)
    }
}
