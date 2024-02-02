use crate::consts::{LAST_BIT, PTR_SIZE_IN_BITS, SECOND_TO_LAST_BIT};
use crate::EncodedPointer;

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

    pointer.set_bool_one(true);
    pointer.set_bool_two(true);
    pointer.set_address(0x200);

    assert_eq!(0x200, pointer.get_address());
    assert_eq!(true, pointer.get_bool_one());
    assert_eq!(true, pointer.get_bool_two());
}
#[test]
#[cfg(target_arch = "x86_64")]
fn const_values() {
    assert_eq!(PTR_SIZE_IN_BITS, 64);
    assert_eq!(LAST_BIT, 63);
    assert_eq!(SECOND_TO_LAST_BIT, 62);
}
#[test]
#[cfg(target_arch = "x86")]
fn const_values() {
    assert_eq!(PTR_SIZE_IN_BITS, 32);
    assert_eq!(LAST_BIT, 31);
    assert_eq!(SECOND_TO_LAST_BIT, 30);
}