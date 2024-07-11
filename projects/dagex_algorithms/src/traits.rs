use core::fmt::Debug;
use std::sync::Arc;

use raf_structural_logging::traits::StructuralLoggerFactory;

/// Represents given algorithm's temporary data.
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

    /// Creates a new [`Algorithm`] with input passed to it.
    /// 
    /// # Errors
    /// This method is responsible for all input validation. For concrete
    /// description see associated [`AlgorithmFactory::Error`] docs.
    fn create<'a>(&mut self, input: Self::Input<'a>)
        -> Result<Self::Algo<'a>, Self::Error>;
}

pub trait AlgorithmFactoryBuilder: Sized + Default {
    type LoggerFactory: StructuralLoggerFactory;
    type AlgoFactory: AlgorithmFactory;
    type Error: Debug;

    /// Sets `logger_factory` for internal usage of algorithm.
    fn set_logger_factory(
        &mut self,
        logger_factory: &Arc<Self::LoggerFactory>);

    /// Creates a new [`AlgorithmFactory`].
    /// 
    /// # Errors
    /// For concrete description see associated [`AlgorithmFactoryBuilder::Error`] docs.
    fn create(self) -> Result<Self::AlgoFactory, Self::Error>;
}
