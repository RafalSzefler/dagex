#![allow(clippy::derivable_impls)]
use crate::{
    cancellation_token_inner::{
        CancellationTokenInner,
        CancellationTokenInnerRegistration},
    callable::Callable,
    pdi,
    IsCancelled};

type CTInner = CancellationTokenInner<pdi::PDIMarkedVector<Callable<'static>>>;
type CTReg = CancellationTokenInnerRegistration<pdi::PDIMarkedVector<Callable<'static>>>;

/// Represents registration of cancellation callback on a token.
/// 
/// # Safety
/// This struct and all its public members are thread safe.
pub struct CancellationTokenRegistration {
    inner: CTReg,
}

impl CancellationTokenRegistration {
    /// Unregisters current registration. Calls to this function after
    /// the first one is noop.
    #[inline(always)]
    pub fn unregister(self) {
        self.inner.unregister();
    }
}


/// Represents cancellation token.
/// 
/// # Safety
/// This struct and all its public members are thread safe.
#[derive(PartialEq, Eq, Hash)]
pub struct CancellationToken {
    inner: CTInner,
}

impl CancellationToken {
    /// Returns empty object if token is cancelled.
    /// 
    /// # Errors
    /// [`IsCancelled`] if the token is already cancelled.
    #[inline(always)]
    pub fn is_cancelled(&self) -> Result<(), IsCancelled> {
        if self.inner.is_cancelled() {
            Err(IsCancelled)
        }
        else
        {
            Ok(())
        }
    }

    /// Register callback to be called on cancellation.
    /// 
    /// # Errors
    /// Returns [`IsCancelled`] if the token is already cancelled. The callback
    /// won't be called in such situation.
    pub fn register<T: FnMut() + 'static>(&mut self, on_cancel: T)
        -> Result<CancellationTokenRegistration, IsCancelled>
    {
        match self.inner.register(on_cancel) {
            Ok(value) => {
                Ok(CancellationTokenRegistration { inner: value })
            },
            Err(err) => Err(err)
        }
    }
}

/// Represents a source of cancellation tokens.
/// 
/// # Safety
/// This struct and all its public members are thread safe.
#[derive(PartialEq, Eq, Hash)]
pub struct CancellationTokenSource {
    inner: CTInner
}

impl CancellationTokenSource {
    /// Retrieves the associated token.
    #[inline(always)]
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            inner: self.inner.clone()
        }
    }

    /// Cancels the token and calls all registration callbacks. Callbacks
    /// are guaranteed to be called exactly once. Returns empty object on
    /// success.
    /// 
    /// # Errors
    /// [`IsCancelled`] if token source is already cancelled.
    #[inline(always)]
    pub fn cancel(&mut self) -> Result<(), IsCancelled> {
        self.inner.cancel()
    }
}

impl Clone for CancellationToken {
    /// Cloning is fast and efficient. Cloned [`CancellationToken`] keeps
    /// pointing to the same token, use this method if you want to share
    /// a source between threads.
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl Default for CancellationTokenSource {
    fn default() -> Self {
        Self { inner: CTInner::default() }
    }
}

impl Clone for CancellationTokenSource {
    /// Cloning is fast and efficient. Cloned [`CancellationTokenSource`] keeps
    /// pointing to the same source, use this method if you want to share
    /// a source between threads.
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

unsafe impl Sync for CancellationTokenSource { }
unsafe impl Send for CancellationTokenSource { }
unsafe impl Sync for CancellationToken { }
unsafe impl Send for CancellationToken { }
unsafe impl Sync for CancellationTokenRegistration { }
unsafe impl Send for CancellationTokenRegistration { }
