use std::mem::size_of;
use crate::encoded::consts::{BOOL_ONE_POSITION, PTR_SIZE_IN_BITS, BOOL_TWO_POSITION};
use crate::encoded::EncodedPointer;

#[test]
fn set_bools() {
    let mut pointer = EncodedPointer::from_address(0x100).unwrap();

    assert_eq!(0x100, pointer.get_address());
    assert_eq!(false, pointer.get_bool_one());
    assert_eq!(false, pointer.get_bool_two());

    pointer.set_bool_one(true);
    pointer.set_bool_two(true);

    assert_eq!(0x100, pointer.get_address());
    assert_eq!(true, pointer.get_bool_one());
    assert_eq!(true, pointer.get_bool_two());
}

#[test]
fn set_address() {
    let mut pointer = EncodedPointer::from_address(0x100).unwrap();

    assert_eq!(0x100, pointer.get_address());
    assert_eq!(false, pointer.get_bool_one());
    assert_eq!(false, pointer.get_bool_two());

    pointer.set_bool_two(true);
    pointer.set_address(0x200);

    assert_eq!(0x200, pointer.get_address());
    assert_eq!(false, pointer.get_bool_one());
    assert_eq!(true, pointer.get_bool_two());

    pointer.set_bool_one(true);
    pointer.set_bool_two(false);
    pointer.set_address(0x300);

    assert_eq!(0x300, pointer.get_address());
    assert_eq!(true, pointer.get_bool_one());
    assert_eq!(false, pointer.get_bool_two());
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_debug() {
    let pointer = EncodedPointer::new(0x100, true, false).unwrap();
    assert_eq!(
        format!("{pointer:?}"),
        "DecodedPointer { pointer: 0x100, bool_one: true, bool_two: false } : 0x8000000000000100"
    );
}
#[test]
#[cfg(target_arch = "x86_64")]
fn assert_values() {

    assert_eq!(size_of::<EncodedPointer>(), size_of::<usize>());
    assert_eq!(PTR_SIZE_IN_BITS, 64);
    assert_eq!(BOOL_ONE_POSITION, 63);
    assert_eq!(BOOL_TWO_POSITION, 62);
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

    // Do something with the buffer
    let byte = unsafe { buffer.read() };
}

#[test]
#[cfg(target_arch = "x86")]
fn assert_values() {
    assert_eq!(PTR_SIZE_IN_BITS, 32);
    assert_eq!(BOOL_ONE_POSITION, 31);
    assert_eq!(BOOL_TWO_POSITION, 30);
}
