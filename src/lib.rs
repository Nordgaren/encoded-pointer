mod consts;
#[cfg(test)]
mod tests;
mod decoded_pointer;

use std::io::{Error, ErrorKind};
use crate::consts::*;
use crate::decoded_pointer::{DecodedPointer, DecodedPointerMut};

/// This is a type that encodes the last two bits of a pointer with bools, to store extra data about a particular
/// pointer. It allows the user to get address portion of the type as a `uintptr_t` or a `*const/mut T`, either of the
/// encoded bools, as well as the entire encoded pointer. It also allows setting of the individual components.
#[derive(Debug, Copy, Clone)]
pub struct EncodedPointer {
    pointer: usize,
}

impl EncodedPointer {
    /// Checks if there is bit collision in the provided pointer, and then returns an EncodedPointer with the given
    /// bool values encoded into the pointer. If there is bit collision, it returns Error with ErrorKind::InvalidInput.
    ///
    /// # Arguments
    ///
    /// * `pointer`: usize
    /// * `bool_one`: bool
    /// * `bool_two`: bool
    ///
    /// returns: Result<EncodedPointer, Error>
    pub fn new(pointer: usize, bool_one: bool, bool_two: bool) -> std::io::Result<Self> {
        if bit_collision(pointer) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Pointer contains data in the two most significant bits, and can't be encoded.",
            ));
        }

        let pointer = pointer | ((bool_one as usize) << LAST_BIT) | ((bool_two as usize) << SECOND_TO_LAST_BIT);
        Ok(EncodedPointer { pointer })
    }
    /// Checks if there is bit collision in the provided pointer, and then returns an EncodedPointer with the encoded
    /// bools set to false. If there is bit collision, it returns std::nullopt.
    ///
    /// # Arguments
    ///
    /// * `address`: usize
    ///
    /// returns: Result<EncodedPointer, Error>
    pub fn from_address(address: usize) -> std::io::Result<Self> {
        Self::new(address, false, false)
    }
    /// Returns an encoded pointer with the provided value, without checking for bit collision. Assumes the given value
    /// is a valid EncodedPointer.
    ///
    /// # Arguments
    ///
    /// * `pointer`: usize
    ///
    /// returns: EncodedPointer
    pub fn from_value_unchecked(pointer: usize) -> EncodedPointer {
        EncodedPointer { pointer }
    }
    /// Returns a DecodedPointer<T> with the specified const pointer type.
    ///
    /// # Arguments
    ///
    /// * `pointer`: usize
    ///
    /// returns: DecodedPointer<T>
    pub fn get_decoded_pointer<T>(self) -> DecodedPointer<T> {
        self.into()
    }
    /// Returns a DecodedPointer<T> with the specified mutable pointer type.
    ///
    /// # Arguments
    ///
    /// * `pointer`: usize
    ///
    /// returns: DecodedPointerMut<T>
    pub fn get_decoded_pointer_mut<T>(self) -> DecodedPointerMut<T> {
        self.into()
    }
    /// Returns the entire encoded pointer value, including the encoded bools.
    ///
    /// returns: usize
    #[inline(always)]
    pub fn get_value(self) -> usize {
        self.pointer
    }
    /// Returns the address portion of the encoded pointer.
    ///
    /// returns: usize
    #[inline(always)]
    pub fn get_address(self) -> usize {
        (self.pointer << 2) >> 2
    }
    /// Returns a const pointer to the generic type, using the address portion of the encoded pointer.
    ///
    /// returns: *const T
    #[inline(always)]
    pub fn get_pointer<T>(self) -> *const T {
        self.get_address() as *const T
    }
    /// Returns a mutable pointer to the generic type, using the address portion of the encoded pointer.
    ///
    /// returns: *mut T
    #[inline(always)]
    pub fn get_mut_pointer<T>(self) -> *mut T {
        self.get_address() as *mut T
    }
    /// Returns the bool encoded into the last bit of the pointer.
    ///
    /// returns: bool
    #[inline(always)]
    pub fn get_bool_one(self) -> bool {
        (self.pointer >> LAST_BIT) != 0
    }
    /// Returns the bool encoded into the second to last bit of the pointer.
    ///
    /// returns: bool
    #[inline(always)]
    pub fn get_bool_two(self) -> bool {
        (self.pointer << 1) >> LAST_BIT != 0
    }
    /// Sets the entire encoded pointer to the given value.
    ///
    /// # Arguments
    ///
    /// * `value`: usize
    #[inline(always)]
    pub fn set_value(&mut self, value: usize) {
        self.pointer = value
    }
    /// Checks if the address has bit collision with the encoded bool portion of the encoded pointer. Sets the _address portion of the encoded
    /// pointer to the address given, if there is no collision, and returns true. Returns false if the address could not
    /// be set.
    ///
    /// # Arguments
    ///
    /// * `pointer`: usize
    ///
    /// returns: bool
    #[inline(always)]
    pub fn set_address(&mut self, pointer: usize) -> bool {
        if bit_collision(pointer) {
            return false;
        }
        let encoded_bools = (self.pointer >> SECOND_TO_LAST_BIT) << SECOND_TO_LAST_BIT;
        self.pointer = pointer | encoded_bools;
        true
    }
    /// Sets the last bit of the encoded pointer to the provided value.
    ///
    /// # Arguments
    ///
    /// * `b`: bool
    pub fn set_bool_one(&mut self, b: bool) {
        self.pointer = (self.pointer << 1) >> 1;
        self.pointer |= (b as usize) << LAST_BIT;
    }
    /// Sets the second to last bit of the encoded pointer to the provided value.
    ///
    /// # Arguments
    ///
    /// * `b`: bool
    #[inline(always)]
    pub fn set_bool_two(&mut self, b: bool) {
        const MASK: usize = !(1 << SECOND_TO_LAST_BIT);
        self.pointer &= MASK;
        self.pointer |= (b as usize) << SECOND_TO_LAST_BIT;
    }
}

/// Returns true if either of the last two bits are set.
///
/// # Arguments
///
/// * `pointer`: usize
fn bit_collision(pointer: usize) -> bool {
    const MASK: usize = 3 << SECOND_TO_LAST_BIT;
    pointer & MASK != 0
}
