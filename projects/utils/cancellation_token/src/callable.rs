use core::ptr::NonNull;

pub(crate) struct Callable<'a> {
    inner: NonNull<dyn FnMut() + 'a>,
}


impl<'a> Callable<'a> {
    pub fn from_owned<'b: 'a, T: FnMut() + 'b>(func: T) -> Self {
        let boxed: Box<dyn FnMut()> = Box::new(func);
        Self::from_box(boxed)
    }

    pub fn from_box<'b: 'a>(boxed: Box<dyn FnMut() + 'b>) -> Self {
        let raw_ptr = Box::into_raw(boxed);
        Self { inner: unsafe { NonNull::new_unchecked(raw_ptr) } }
    }

    #[inline(always)]
    pub fn call(&mut self) {
        let reff = unsafe { self.inner.as_mut() };
        reff();
    }
}

impl<'a> Drop for Callable<'a> {
    fn drop(&mut self) {
        let _boxed = unsafe { Box::from_raw(self.inner.as_ptr()) };
    }
}

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
        let mut callable = Callable::from_owned(move || {
            let mut locked = clone.lock().unwrap();
            *locked += 1;
        });

        assert_eq!(get_value(), 1);
        callable.call();
        assert_eq!(get_value(), 2);
        callable.call();
        assert_eq!(get_value(), 3);
        callable.call();
        assert_eq!(get_value(), 4);
        callable.call();
        assert_eq!(get_value(), 5);
        callable.call();
        assert_eq!(get_value(), 6);
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

        let mut callable: Callable = Callable::from_owned(&process);

        assert_eq!(get(), 0);
        callable.call();
        assert_eq!(get(), 5);
    }
}
