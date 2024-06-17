use tagged_pointer::{Bit, BitConstructionError};

#[test]
fn test_zero_comparison() {
    let bit = unsafe { Bit::new_unchecked(0) };
    assert_eq!(bit, Bit::ZERO);
}

#[test]
fn test_one_comparison() {
    let bit = unsafe { Bit::new_unchecked(1) };
    assert_eq!(bit, Bit::ONE);
}

#[test]
fn test_and() {
    assert_eq!(Bit::ZERO & Bit::ZERO, Bit::ZERO);
    assert_eq!(Bit::ZERO & Bit::ONE, Bit::ZERO);
    assert_eq!(Bit::ONE & Bit::ZERO, Bit::ZERO);
    assert_eq!(Bit::ONE & Bit::ONE, Bit::ONE);
}

#[test]
fn test_or() {
    assert_eq!(Bit::ZERO | Bit::ZERO, Bit::ZERO);
    assert_eq!(Bit::ZERO | Bit::ONE, Bit::ONE);
    assert_eq!(Bit::ONE | Bit::ZERO, Bit::ONE);
    assert_eq!(Bit::ONE | Bit::ONE, Bit::ONE);
}

#[test]
fn test_xor() {
    assert_eq!(Bit::ZERO ^ Bit::ZERO, Bit::ZERO);
    assert_eq!(Bit::ZERO ^ Bit::ONE, Bit::ONE);
    assert_eq!(Bit::ONE ^ Bit::ZERO, Bit::ONE);
    assert_eq!(Bit::ONE ^ Bit::ONE, Bit::ZERO);
}

#[test]
fn test_out_of_range() {
    for value in 2..255u8 {
        assert!(matches!(Bit::new(value), Err(BitConstructionError::ValueOutOfRange)));
    }
}

#[test]
fn test_valid_values() {
    assert_eq!(Bit::new(0).unwrap(), Bit::ZERO);
    assert_eq!(Bit::new(1).unwrap(), Bit::ONE);
}

