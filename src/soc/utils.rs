//! Miscelleneous utilities for SoC functions (private)

pub fn set_bit(inval: u32, bit: u8, state: bool) -> u32 {
    if state {
        inval | (1 << u32::from(bit))
    } else {
        inval & !(1 << u32::from(bit))
    }
}

pub fn get_bit(inval: u32, bit: u8) -> bool {
    (inval & (1 << u32::from(bit))) != 0
}

#[test]
fn test_set_bit() {
    assert_eq!(set_bit(0x7FFFFFFF, 31, true), 0xFFFFFFFF);
}

#[test]
fn test_get_bit() {
    let pattern: u32 = 0xAAAAAAAA;
    assert!(get_bit(pattern, 1));
    assert!(get_bit(pattern, 3));
    assert!(get_bit(pattern, 5));
    assert!(get_bit(pattern, 7));
    assert!(get_bit(pattern, 9));
    assert!(get_bit(pattern, 11));
    assert!(get_bit(pattern, 13));
    assert!(get_bit(pattern, 15));
    assert!(get_bit(pattern, 17));
    assert!(get_bit(pattern, 19));
    assert!(get_bit(pattern, 21));
    assert!(get_bit(pattern, 23));
    assert!(get_bit(pattern, 25));
    assert!(get_bit(pattern, 27));
    assert!(get_bit(pattern, 29));
    assert!(get_bit(pattern, 31));
}
