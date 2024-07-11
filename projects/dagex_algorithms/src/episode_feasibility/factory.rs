use std::{marker::PhantomData, sync::Arc};

use raf_structural_logging::core::CoreLoggerFactory;

use crate::traits::{AlgorithmFactory, AlgorithmFactoryBuilder};

use super::{EpisodeFeasabilityAlgorithm, EpisodeFeasabilityInput};

type EFLoggerFactory = CoreLoggerFactory;

pub struct EpisodeFeasabilityAlgorithmFactory {
    _phantom: PhantomData<()>
}

impl EpisodeFeasabilityAlgorithmFactory {
    pub(super) fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl AlgorithmFactory for EpisodeFeasabilityAlgorithmFactory {
    type Input<'a> = EpisodeFeasabilityInput<'a>;

    type Algo<'a> = EpisodeFeasabilityAlgorithm<'a>;

    type Error = ();

    fn create<'a>(&mut self, input: Self::Input<'a>)
        -> Result<Self::Algo<'a>, Self::Error>
    {
        Ok(Self::Algo::new(input))
    }
}

#[derive(Default)]
pub struct EpisodeFeasabilityAlgorithmFactoryBuilder {
    _phantom: PhantomData<()>,
}

impl AlgorithmFactoryBuilder for EpisodeFeasabilityAlgorithmFactoryBuilder {
    type LoggerFactory = EFLoggerFactory;

    type AlgoFactory = EpisodeFeasabilityAlgorithmFactory;

    type Error = ();

    fn set_logger_factory(
        &mut self,
        _logger_factory: &Arc<Self::LoggerFactory>)
    {
    }

    fn create(self) -> Result<Self::AlgoFactory, Self::Error> {
        Ok(Self::AlgoFactory::new())
    }
}
