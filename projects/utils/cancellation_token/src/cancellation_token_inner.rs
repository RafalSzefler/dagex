use core::hash::{Hash, Hasher};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

use crate::{callable::Callable, pdi::{self, PDIItemIndicator}, IsCancelled};

pub(crate) trait Pdi : pdi::PDICollection<Item = Callable<'static>> { }

impl<T: pdi::PDICollection<Item = Callable<'static>>> Pdi for T { }

struct CancellationTokenData<T: Pdi> {
    pub strong_counter: AtomicU32,
    pub is_cancelled: AtomicBool,
    pub on_cancel: T,
    pub lock: Mutex<()>,
}

pub(crate) struct CancellationTokenInner<T: Pdi> {
    ptr: *mut CancellationTokenData<T>
}

impl<T: Pdi> PartialEq for CancellationTokenInner<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.ptr, other.ptr)
    }
}

impl<T: Pdi> Eq for CancellationTokenInner<T> { }

impl<T: Pdi> Hash for CancellationTokenInner<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

pub(crate) struct CancellationTokenInnerRegistration<T: Pdi>
    where T : 'static
{
    pub indicator: T::Indicator<'static>,
}

impl<T: Pdi> CancellationTokenInnerRegistration<T> {
    #[inline(always)]
    pub fn unregister(self) {
        self.indicator.remove();
    }
}


impl<T: Pdi> CancellationTokenInner<T> {
    pub fn cancel(&mut self) -> Result<(), IsCancelled> {
        if self.is_cancelled() {
            return Err(IsCancelled);
        }

        let data = self.get_ref();

        let mut to_remove = T::default();

        {
            let mut _guard = data.lock.lock().unwrap();
            if self.is_cancelled() {
                return Err(IsCancelled);
            }
            data.is_cancelled.store(true, Ordering::Release);
            to_remove.swap(&mut data.on_cancel);
        }

        for mut callback in to_remove {
            callback.call();
        }

        Ok(())
    }

    #[inline(always)]
    pub fn register<F: FnMut() + 'static>(&mut self, on_cancel: F)
        -> Result<CancellationTokenInnerRegistration<T>, IsCancelled>
    {
        if self.is_cancelled() {
            return Err(IsCancelled);
        }

        let callback = Callable::from_owned(on_cancel);
        let static_self: &mut CancellationTokenInner<T> = unsafe { &mut *core::ptr::from_mut::<Self>(self) };
        let mut _guard = static_self.get_ref().lock.lock().unwrap();
        if static_self.is_cancelled() {
            return Err(IsCancelled);
        }
        let indicator = static_self.get_ref().on_cancel.push(callback);
        let result = CancellationTokenInnerRegistration {
            indicator: indicator
        };
        Ok(result)
    }

    #[inline(always)]
    pub fn is_cancelled(&self) -> bool {
        self.get_ref().is_cancelled.load(Ordering::Acquire)
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn get_ref(&self) -> &mut CancellationTokenData<T>{
        unsafe { &mut *self.ptr }
    }
}

impl<T: Pdi> Drop for CancellationTokenInner<T> {
    fn drop(&mut self) {
        let prev_value = self.get_ref().strong_counter.fetch_sub(1, Ordering::Relaxed);
        if prev_value == 1 {
            let _boxed = unsafe { Box::from_raw(self.ptr) };
        }
    }
}

impl<T: Pdi> Clone for CancellationTokenInner<T> {
    fn clone(&self) -> Self {
        self.get_ref().strong_counter.fetch_add(1, Ordering::Relaxed);
        Self { ptr: self.ptr }
    }
}

impl<T: Pdi> Default for CancellationTokenInner<T> {
    fn default() -> Self {
        let data = CancellationTokenData {
            strong_counter: AtomicU32::new(1),
            is_cancelled: AtomicBool::new(false),
            on_cancel: T::default(),
            lock: Mutex::new(()),
        };
        let boxed = Box::new(data);
        Self {
            ptr: Box::into_raw(boxed),
        }
    }
}
