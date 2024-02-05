# Encoded Pointer
[GitHub](https://github.com/Nordgaren/encoded-pointer)  
An encoded pointer data structure that encodes up to two bools in the last two bits of a pointer.

## Usage
 This is a type that encodes the last two bits of a pointer with bools, to store extra data about a particular
 pointer. It allows the user to get address portion of the type as a `usize` or a `*const/mut T`, either of the
 encoded bools, as well as the entire encoded pointer. It also allows setting of the individual components.

 The user can also choose to use the `DecodedPointer<T>` and `DecodedPointerMut<T>` structs to deconstruct the encoded
 pointer to it's base parts.

 This type also implements `Copy` and `Clone`, so can be passed by value without having issues with the borrow checker.
 It also implements `From<EncodedPointer>` for `*mut T` and `*const T`, as well as `Debug` for convenience.

## Example

 ```rust
use encoded_pointer::encoded::EncodedPointer;
use encoded_pointer::decoded::DecodedPointer;

 fn example(some_address: usize) {
     let mut encoded = EncodedPointer::from_address(some_address)
             .expect("Could not encode pointer");
     let pointer = encoded.get_pointer();
     let bool_one = check_some_value(pointer);
     encoded.set_bool_one(bool_one);

     let bool_two = check_some_other_value(pointer);
     encoded.set_bool_two(bool_two);

     inner_example(encoded);
 }

 fn inner_example(pointer: EncodedPointer) {
     let DecodedPointer {
         pointer: buffer,
         bool_one: something_i_care_about,
         bool_two: something_else_i_care_about,
     } = pointer.get_decoded::<u8>();
     
     if something_i_care_about {
         // Do something
     }

     if something_else_i_care_about {
         // Do something
     }

     // Do something with the pointer
     let byte = unsafe { buffer.read() };
 }
# // Dummy functions for documentation examples
# fn check_some_value(pointer: *const u8) -> bool {
#   true
# }
# fn check_some_other_value(pointer: *const u8) -> bool {
#   true
# }
 ```

## Future
x86_64 defines a 48-bit virtual address space. This means that we can use up to 16 bits to encode extra data into the
pointer. This may change in the future, though, so I am not sure what my plans are. It might be a while before we get to
the point where we are using that much memory at once, and some industries will not be able to use those extra 16 bits 
sooner than others. At this point, though, we could encode a 16-bit number into the pointer, and use it, if we needed.

I may update this some day to use a macro so that the user can generate their own custom types to be stored in those 16 
bits, splitting the bits up as the user desires (maybe you need a 4 bit number and a few bools, or an entire byte and some
bools, or multiple 4-5 bit numbers).

Currently, this structure is generally compatible with 32-bit Windows, due to the use of only two encoded bools. I 
think I can expand this number to 4, but, the address space in x86 is not hardware limited, like it is in x86_64. Two 
encoded bools may be too much, though, as even on Windows, the address space only leaves the sign bit untouched. For most
applications in 32-bit OSs, the last two bits should be usable. A few programs may use the last bit, though, if they need
a lot of memory. You could get around this if you made sure that the pointers you used this on were allocated in a specific
range (Any address less than 0x40000000). System modules will probably be out of range, but most userspace allocations
and modules, even with ASLR, should be fine.

I don't know the compatability on 32-bit Linux, as I don't do much RE on Linux, so I don't tend to see linux addresses 
and their ranges, so I couldn't even guess, right now. If anyone has some resources, I'd appreciate it (does 32-bit Linux 
use the signed bit of a pointer sized type?). It should be compatible in 64-bit Linux, right now, as the virtual address 
space is hardware limited to 48 bits on 64-bit systems.  
