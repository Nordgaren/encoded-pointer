use crate::encoded::EncodedPointer;

/// A helper type used to deconstruct an `EncodedPointer` to a `*mut T` and two encoded bools.
/// Does not have any methods associated with it, and all fields are public.
///
/// # Example
///
/// ```rust
/// # use encoded_pointer::encoded::EncodedPointer;
/// # use encoded_pointer::decoded::DecodedPointer;
/// fn example(pointer: EncodedPointer) {
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
#[derive(Debug)]
pub struct DecodedPointer<T> {
    pub pointer: *const T,
    pub bool_one: bool,
    pub bool_two: bool,
}

/// A type used to deconstruct an `EncodedPointer` to a `*mut T` and two encoded bools.
/// Does not have any methods associated with it, and all fields are public.
///
/// # Example
///
/// ```rust
/// # use encoded_pointer::encoded::EncodedPointer;
/// # use encoded_pointer::decoded::DecodedPointerMut;
/// fn example(pointer: &mut EncodedPointer) {
///     let DecodedPointerMut {
///         pointer: mut_buffer,
///         bool_one: something_i_care_about,
///         bool_two: something_else_i_care_about,
///     } = pointer.get_decoded_mut::<u8>();
///
///     if something_i_care_about {
///         // Do something
///     }
///
///     if something_else_i_care_about {
///         // Do something
///     }
///
///     // Do something with the mutable pointer
///     unsafe { mut_buffer.write(0) };
/// }
/// ```
#[derive(Debug)]
pub struct DecodedPointerMut<T> {
    pub pointer: *mut T,
    pub bool_one: bool,
    pub bool_two: bool,
}

impl<T> From<&EncodedPointer> for DecodedPointer<T> {
    fn from(pointer: &EncodedPointer) -> Self {
        DecodedPointer {
            pointer: pointer.get_pointer(),
            bool_one: pointer.get_bool_one(),
            bool_two: pointer.get_bool_two(),
        }
    }
}

impl<T> From<&mut EncodedPointer> for DecodedPointerMut<T> {
    fn from(pointer: &mut EncodedPointer) -> Self {
        DecodedPointerMut {
            pointer: pointer.get_pointer_mut(),
            bool_one: pointer.get_bool_one(),
            bool_two: pointer.get_bool_two(),
        }
    }
}
