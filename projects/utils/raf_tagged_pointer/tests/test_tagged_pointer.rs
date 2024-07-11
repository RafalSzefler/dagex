use core::mem::{size_of, size_of_val};
use std::sync::{Arc, Mutex};

use raf_tagged_pointer::{Bit, ConstructionError, TaggedPointer};

type FixedTaggedPointer<T> = TaggedPointer<T, 2>;

#[test]
fn test_misalignement() {
    let mut value = 15;
    let raw_ptr = &mut value as *mut i32;
    assert!(matches!(TaggedPointer::<i32, 16>::new(raw_ptr), Err(ConstructionError::PointerMisaligned)));
}

#[test]
fn test_ptr_thin() {
    let mut value = 15;
    let raw_ptr = &mut value as *mut i32;
    let mut tagged = FixedTaggedPointer::new(raw_ptr).unwrap();

    let mut for_call = tagged.clone();

    assert_eq!(size_of_val(&for_call), size_of::<*mut i32>());

    let mut get_it = move || {
        *unsafe { for_call.deref_mut() }
    };

    for _ in 0..100 {
        // Test bit 0
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert_eq!(get_it(), value);

        tagged.set_n_bit::<0>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ONE);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert_eq!(get_it(), value);

        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert_eq!(get_it(), value);

        // test bit 1
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert_eq!(get_it(), value);

        tagged.set_n_bit::<1>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ONE);
        assert_eq!(get_it(), value);

        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert_eq!(get_it(), value);
    }
}


#[inline(never)]
fn build_func(arc: Arc<Mutex<i32>>) -> Box<dyn FnMut()> {
    Box::new(move || {
        let mut guard = arc.lock().unwrap();
        *guard += 1;
    })
}

#[test]
fn test_ptr_fat_fnmut() {
    let holder = Arc::new(Mutex::new(1));

    let get_clone = holder.clone();
    let get = || {
        let guard = get_clone.lock().unwrap();
        *guard
    };

    let mut func = build_func(holder.clone());

    assert_eq!(get(), 1);
    func();
    assert_eq!(get(), 2);

    let raw_ptr = Box::into_raw(func);
    let mut tagged = FixedTaggedPointer::new(raw_ptr).unwrap();

    let mut for_call = tagged.clone();

    assert!(size_of::<*mut dyn FnMut()>() > size_of::<*mut fn()>());
    assert_eq!(size_of_val(&for_call), size_of::<*mut dyn FnMut()>());

    let mut call_it = move || {
        let func = unsafe { for_call.deref_mut() };
        func()
    };

    let mut idx = 2;
    for _ in 0..100 {
        // Test bit 0
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);

        tagged.set_n_bit::<0>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ONE);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);

        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);

        // test bit 1
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);

        tagged.set_n_bit::<1>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ONE);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);

        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        call_it();
        idx += 1;
        assert_eq!(get(), idx);
    }

    let _tmp = unsafe { Box::from_raw(raw_ptr) };
}



#[test]
fn test_ptr_fat_slice() {
    let slice = &mut [1i32, 3, 2, 7, 1, 0, 1, 15];
    let slice_ref = slice as &mut [i32];
    let raw_ptr = slice_ref as *mut [i32];

    let mut tagged = FixedTaggedPointer::new(raw_ptr).unwrap();

    assert!(size_of::<*mut [i32]>() > size_of::<*mut i32>());
    assert_eq!(size_of_val(&tagged), size_of::<*mut [i32]>());

    let clone = tagged.clone();
    let call_it = move || {
        let arr = unsafe { clone.deref() };
        arr == slice
    };

    for _ in 0..100 {
        // Test bit 0
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert!(call_it());

        tagged.set_n_bit::<0>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ONE);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert!(call_it());

        tagged.set_n_bit::<0>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert!(call_it());

        // test bit 1
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert!(call_it());

        tagged.set_n_bit::<1>(Bit::ONE);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ONE);
        assert!(call_it());

        tagged.set_n_bit::<1>(Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<0>(), Bit::ZERO);
        assert_eq!(tagged.get_n_bit::<1>(), Bit::ZERO);
        assert!(call_it());
    }
}
