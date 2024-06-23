#![allow(clippy::derivable_impls)]

use core::fmt::{Debug, Formatter};
use crate::{
    cancellation_token_inner::{
        CancellationTokenInner,
        CancellationTokenInnerRegistration},
    callable::Callable,
    pdi,
    TokenState};

type CTInner = CancellationTokenInner<pdi::PDIMarkedVector<Callable<'static>>>;
type CTReg = CancellationTokenInnerRegistration<pdi::PDIMarkedVector<Callable<'static>>>;

/// Represents registration of cancellation callback on a token.
/// 
/// # Safety
/// This struct and all its public members are thread safe.
pub struct CancellationTokenRegistration {
    inner: CTReg,
}

impl Debug for CancellationTokenRegistration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CancellationTokenRegistration").finish()
    }
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

impl Debug for CancellationToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CancellationToken")
            .field("id", &self.id()).finish()
    }
}

pub struct RegistrationError<T>
{
    pub on_cancel: T,
    pub state: TokenState,
}

impl CancellationToken {
    pub fn id(&self) -> usize {
        self.inner.id()
    }

    /// Retrieves current state of the token.
    #[inline(always)]
    pub fn get_state(&self) -> TokenState {
        self.inner.get_state()
    }

    /// Register callback to be called on cancellation.
    /// 
    /// # Errors
    /// `on_cancel` callback won't be called on errors, and will be return
    /// together with specific errors:
    /// * [`TokenState::IsCancelled`] if the token is already cancelled.
    /// * [`TokenState::NotCancellable`] if the token is not cancellable.
    pub fn register<T: FnMut() + 'static>(&mut self, on_cancel: T)
        -> Result<CancellationTokenRegistration, RegistrationError<T>>
    {
        match self.inner.register(on_cancel) {
            Ok(value) => {
                Ok(CancellationTokenRegistration { inner: value })
            },
            Err((on_cancel, state))
                => Err(RegistrationError { on_cancel, state})
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
    pub fn id(&self) -> usize {
        self.inner.id()
    }

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
    /// [`TokenState`] if token source is already cancelled.
    #[inline(always)]
    pub fn cancel(&mut self) -> Result<(), TokenState> {
        self.inner.cancel()
    }
}

impl Debug for CancellationTokenSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CancellationTokenSource")
            .field("id", &self.id()).finish()
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

impl Default for CancellationToken {
    /// Create a non-cancellable token, not associated with any source.
    fn default() -> Self {
        Self { inner: CTInner::create_not_cancellable() }
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
