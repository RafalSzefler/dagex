use tagged_pointer::{Bit, TaggedPointer};

pub(crate) struct CallableWithFlag<'a> {
    inner: TaggedPointer<dyn FnMut() + 'a, 2>,
}

const OWNERSHIP_POS: usize = 0;
const DATA_POS: usize = 1;

impl<'a> CallableWithFlag<'a> {
    pub fn from_owned<T: FnMut() + 'a>(func: T) -> Self {
        let boxed: Box<dyn FnMut() + 'a> = Box::new(func);
        let raw_ptr = Box::into_raw(boxed);
        let mut tagged = TaggedPointer::new(raw_ptr).unwrap();
        tagged.set_n_bit::<OWNERSHIP_POS>(Bit::ONE);
        tagged.set_n_bit::<DATA_POS>(Bit::ZERO);
        Self { inner: tagged }
    }

    pub unsafe fn from_raw(func: *const dyn FnMut()) -> Self {
        let raw_ptr = func as *mut dyn FnMut();
        let mut tagged = TaggedPointer::new(raw_ptr).unwrap();
        tagged.set_n_bit::<OWNERSHIP_POS>(Bit::ZERO);
        tagged.set_n_bit::<DATA_POS>(Bit::ZERO);
        Self { inner: tagged }
    }

    #[inline(always)]
    pub fn call(&mut self) {
        let bref = unsafe { self.inner.deref_mut() };
        bref();
    }

    #[inline(always)]
    pub fn set_flag(&mut self, value: bool) {
        let bit = unsafe { Bit::new_unchecked(value as u8) };
        self.inner.set_n_bit::<DATA_POS>(bit);
    }

    #[inline(always)]
    pub fn get_flag(&self) -> bool {
        let bit = self.inner.get_n_bit::<DATA_POS>();
        bit == Bit::ONE
    }
}

impl<'a> Drop for CallableWithFlag<'a> {
    fn drop(&mut self) {
        let ownership_bit = self.inner.get_n_bit::<OWNERSHIP_POS>();
        if ownership_bit == Bit::ONE {
            let raw_ptr = self.inner.as_ptr_mut();
            let _boxed = unsafe { Box::from_raw(raw_ptr) };
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_simple_call() {
        let counter = Arc::new(Mutex::new(1));
        let clone = counter.clone();
        let get_value = || {
            let locked = clone.lock().unwrap();
            *locked
        };

        let clone: Arc<Mutex<i32>> = counter.clone();
        let mut callable = CallableWithFlag::from_owned(move || {
            let mut locked = clone.lock().unwrap();
            *locked += 1;
        });

        assert_eq!(get_value(), 1);
        callable.call();
        assert_eq!(get_value(), 2);
        assert_eq!(callable.get_flag(), false);
        callable.call();
        assert_eq!(get_value(), 3);
        callable.set_flag(true);
        callable.call();
        assert_eq!(get_value(), 4);
        assert_eq!(callable.get_flag(), true);
        callable.call();
        assert_eq!(get_value(), 5);
        callable.set_flag(false);
        callable.call();
        assert_eq!(get_value(), 6);
        assert_eq!(callable.get_flag(), false);
    }

    #[test]
    fn test_not_owned_call() {
        let counter = Arc::new(Mutex::new(1));
        let clone = counter.clone();
        let get_value = || {
            let locked = clone.lock().unwrap();
            *locked
        };

        let clone: Arc<Mutex<i32>> = counter.clone();
        let raw_callable = move || {
            let mut locked = clone.lock().unwrap();
            *locked += 1;
        };

        let mut callable = unsafe { CallableWithFlag::from_raw(&raw_callable) };

        assert_eq!(get_value(), 1);
        callable.call();
        assert_eq!(get_value(), 2);
        assert_eq!(callable.get_flag(), false);
        callable.call();
        assert_eq!(get_value(), 3);
        callable.set_flag(true);
        callable.call();
        assert_eq!(get_value(), 4);
        assert_eq!(callable.get_flag(), true);
        callable.call();
        assert_eq!(get_value(), 5);
        callable.set_flag(false);
        callable.call();
        assert_eq!(get_value(), 6);
        assert_eq!(callable.get_flag(), false);

        drop(raw_callable);
    }

    #[test]
    fn test_static_call() {
        static mut VALUE: i32 = 0;
        fn get() -> i32 {
            unsafe { VALUE }
        }

        fn process() {
            unsafe { VALUE = 5 };
        }

        let mut callable = unsafe {
            CallableWithFlag::from_raw(&process) };

        assert_eq!(get(), 0);
        callable.call();
        assert_eq!(get(), 5);
    }
}
