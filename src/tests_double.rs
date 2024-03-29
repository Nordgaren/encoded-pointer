use crate::encoded::consts::{BOOL_ONE_POSITION, BOOL_TWO_POSITION, PTR_SIZE_IN_BITS};
use crate::encoded::EncodedPointer;

#[test]
/// Check that calling the set method actually changes the bools in the `EncodedPointer`, and doesn't change the address
/// portion of the `EncodedPointer`
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
/// Check that we can set the address portion of the `EncodedPointer` without effecting the encoded bools.
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
/// Test the debug output and make sure it's in the right format.
#[cfg(target_arch = "x86_64")]
fn test_debug() {
    let pointer = EncodedPointer::new(0x100, true, false).unwrap();
    assert_eq!(
        format!("{pointer:?}"),
        "DecodedPointer { pointer: 0x100, bool_one: true, bool_two: false } : 0x8000000000000100"
    );
}
#[test]
#[cfg(target_arch = "x86")]
/// Test the debug output and make sure it's in the right format.
fn test_debug() {
    let pointer = EncodedPointer::new(0x100, true, false).unwrap();
    assert_eq!(
        format!("{pointer:?}"),
        "DecodedPointer { pointer: 0x100, bool_one: true, bool_two: false } : 0x80000100"
    );
}
#[test]
#[cfg(target_arch = "x86_64")]
/// Assert the positions of the bits are correct.
fn assert_values() {
    assert_eq!(PTR_SIZE_IN_BITS, 64);
    assert_eq!(BOOL_ONE_POSITION, 63);
    assert_eq!(BOOL_TWO_POSITION, 62);
}

#[test]
#[cfg(target_arch = "x86")]
/// Assert the positions of the bits are correct.
fn assert_values() {
    assert_eq!(PTR_SIZE_IN_BITS, 32);
    assert_eq!(BOOL_ONE_POSITION, 31);
    assert_eq!(BOOL_TWO_POSITION, 30);
}
