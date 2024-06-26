use core::hash::{Hash, Hasher};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::ptr::{self, null_mut};
use std::sync::Mutex;

use crate::TokenState;
use crate::{callable::Callable, pdi::{self, PDIItemIndicator}};

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
    pub fn id(&self) -> usize {
        let addr = ptr::addr_of!(self.ptr).cast::<usize>();
        unsafe { *addr }
    }

    pub fn create_not_cancellable() -> Self {
        Self { ptr: null_mut() }
    }

    pub fn cancel(&mut self) -> Result<(), TokenState> {
        match self.get_state() {
            TokenState::Ok => { }
            err => return Err(err)
        }

        let data = self.get_ref();

        let mut to_remove = T::default();

        {
            let mut _guard = data.lock.lock().unwrap();
            if self.get_state() == TokenState::IsCancelled {
                return Err(TokenState::IsCancelled);
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
        -> Result<CancellationTokenInnerRegistration<T>, (F, TokenState)>
    {
        match self.get_state() {
            TokenState::Ok => { },
            err => return Err((on_cancel, err))
        }

        let static_self: &mut CancellationTokenInner<T> = unsafe { &mut *core::ptr::from_mut::<Self>(self) };

        let indicator;

        {
            let mut _guard = static_self.get_ref().lock.lock().unwrap();
            if static_self.get_state() == TokenState::IsCancelled {
                return Err((on_cancel, TokenState::IsCancelled));
            }
            let callback = Callable::from_owned(on_cancel);
            indicator = static_self.get_ref().on_cancel.push(callback);
        }

        let result = CancellationTokenInnerRegistration {
            indicator: indicator
        };
        Ok(result)
    }

    #[inline(always)]
    pub fn get_state(&self) -> TokenState {
        if ptr::eq(self.ptr, null_mut()) {
            return TokenState::NotCancellable;
        }

        if self.get_ref().is_cancelled.load(Ordering::Acquire) {
            return TokenState::IsCancelled;
        }
        
        TokenState::Ok
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn get_ref(&self) -> &mut CancellationTokenData<T>{
        unsafe { &mut *self.ptr }
    }
}

impl<T: Pdi> Drop for CancellationTokenInner<T> {
    fn drop(&mut self) {
        if self.get_state() == TokenState::NotCancellable {
            return;
        }

        let prev_value = self.get_ref().strong_counter.fetch_sub(1, Ordering::Relaxed);
        if prev_value == 1 {
            let _boxed = unsafe { Box::from_raw(self.ptr) };
        }
    }
}

impl<T: Pdi> Clone for CancellationTokenInner<T> {
    fn clone(&self) -> Self {
        if self.get_state() == TokenState::NotCancellable {
            return Self { ptr: null_mut() };
        }
        
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
