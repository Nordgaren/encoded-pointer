use crate::EncodedPointer;

/// A type used to deconstruct an EncodedPointer to `*const T` and two encoded bools.
pub struct DecodedPointer<T> {
    pub pointer: *const T,
    pub bool_one: bool,
    pub bool_two: bool,
}
/// A type used to deconstruct an EncodedPointer to a `*mut T` and two encoded bools.
pub struct DecodedPointerMut<T> {
    pub pointer: *mut T,
    pub bool_one: bool,
    pub bool_two: bool,
}
impl<T> From<EncodedPointer> for DecodedPointer<T> {
    fn from(pointer: EncodedPointer) -> Self {
        DecodedPointer {
            pointer: pointer.get_pointer(),
            bool_one: pointer.get_bool_one(),
            bool_two: pointer.get_bool_two(),
        }
    }
}

impl<T> From<EncodedPointer> for DecodedPointerMut<T> {
    fn from(pointer: EncodedPointer) -> Self {
        DecodedPointerMut {
            pointer: pointer.get_mut_pointer(),
            bool_one: pointer.get_bool_one(),
            bool_two: pointer.get_bool_two(),
        }
    }
}
