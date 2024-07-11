use crate::IsCancelled;

pub trait CancellationTokenRegistration : Sync + Send {
    /// Unregisters current registration. Calls to this function after
    /// the first one is noop.
    fn unregister(&mut self);
}

pub trait CancellationToken<'a> : Clone + Sync + Send {
    /// Checks whether token is cancelled.
    fn try_it(&self) -> Result<(), IsCancelled>;

    /// Register callback to be called on cancellation. If the token is
    /// cancelled, the callback will becalled immediately.
    fn register<T: FnMut() + 'a>(&mut self, on_cancel: T)
        -> Result<impl CancellationTokenRegistration, IsCancelled>;
}

pub trait CancellationTokenSource<'a> : CancellationToken<'a> {
    /// Retrieves the associated token.
    fn get_token(&self) -> impl CancellationToken;
    
    /// Cancels the token and calls all registration callbacks. Result is
    /// Ok if the token has been cancelled, and Err if it already was
    /// cancelled.
    fn cancel(&mut self) -> Result<(), IsCancelled>;
}
