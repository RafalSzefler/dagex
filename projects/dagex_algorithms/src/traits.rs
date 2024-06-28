use core::fmt::Debug;

pub trait Algorithm<'a>: Sized {
    type Input<'b>;
    type Output<'b>;
    type Error: Debug;

    /// Runs current algorithm on the internal input and consumes
    /// the [`Algorithm`] instance.
    /// 
    /// # Errors
    /// For errors see [`Algorithm::Error`] description.
    fn run(self) -> Result<Self::Output<'a>, Self::Error>;
}

pub trait AlgorithmFactory: Sized {
    type Input<'a>;
    type Algo<'a>: Algorithm<'a, Input<'a>=Self::Input<'a>>;
    type Error: Debug;

    fn new() -> Self;

    /// Creates a new [`Algorithm`] with input passed to it.
    /// 
    /// # Errors
    /// This method is responsible for all input validation.For concrete
    /// description see associated [`AlgorithmFactory::Error`] docs.
    fn create<'a>(&mut self, input: Self::Input<'a>)
        -> Result<Self::Algo<'a>, Self::Error>;
}
