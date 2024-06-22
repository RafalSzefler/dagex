use array::Array;

#[test]
fn test_u8_array_1() {
    let array = Array::<u8>::new(0);
    let slice = array.as_slice();
    assert_eq!(slice.len(), 0);
    assert!(slice.get(0).is_none());
}

#[test]
fn test_u8_array_2() {
    let mut array = Array::<u8>::new(3);
    let slice = array.as_slice_mut();
    slice[0] = 1;
    slice[1] = 15;
    slice[2] = 3;
    assert_eq!(array.as_slice(), &[1, 15, 3]);
}

struct Baz {
    pub value: i32,
}

impl Default for Baz {
    fn default() -> Self {
        Self { value: 7 }
    }
}

#[test]
fn test_baz_array() {
    let array = Array::<Baz>::new(3);
    let slice = array.as_slice();
    for item in slice {
        assert_eq!(item.value, 7);
    }
}
