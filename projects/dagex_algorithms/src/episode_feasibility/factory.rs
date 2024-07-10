use std::{hash::{Hash, Hasher}, sync::Arc};

use immutable_string::imm;
use structural_logging::{
    traits::StructuralLoggerFactory,
    core::CoreLoggerFactory};

use crate::{logger::build_default_logger_factory, traits::{AlgorithmFactory, AlgorithmFactoryBuilder}};

use super::{EpisodeFeasabilityAlgorithm, EpisodeFeasabilityInput};

type EFLoggerFactory = CoreLoggerFactory;

pub struct EpisodeFeasabilityAlgorithmFactory {
    logger_factory: Arc<EFLoggerFactory>
}

impl EpisodeFeasabilityAlgorithmFactory {
    pub(super) fn new(logger_factory: Arc<EFLoggerFactory>) -> Self {
        Self { logger_factory }
    }
}

impl AlgorithmFactory for EpisodeFeasabilityAlgorithmFactory {
    type Input<'a> = EpisodeFeasabilityInput<'a>;

    type Algo<'a> = EpisodeFeasabilityAlgorithm<'a>;

    type Error = ();

    fn create<'a>(&mut self, input: Self::Input<'a>)
        -> Result<Self::Algo<'a>, Self::Error>
    {
        let mut hasher = fnv1a_hasher::FNV1a32Hasher::new();
        input.hash(&mut hasher);
        let value = hasher.finish();
        let name = "EF_".to_owned() + &value.to_string();
        let logger = self.logger_factory.create(&imm!(name.as_str()));
        Ok(Self::Algo::new(input, logger))
    }
}

#[derive(Default)]
pub struct EpisodeFeasabilityAlgorithmFactoryBuilder {
    logger_factory: Option<Arc<EFLoggerFactory>>
}

impl AlgorithmFactoryBuilder for EpisodeFeasabilityAlgorithmFactoryBuilder {
    type LoggerFactory = EFLoggerFactory;

    type AlgoFactory = EpisodeFeasabilityAlgorithmFactory;

    type Error = ();

    fn set_logger_factory(
        &mut self,
        logger_factory: &Arc<Self::LoggerFactory>)
    {
        self.logger_factory = Some(logger_factory.clone());
    }

    fn create(self) -> Result<Self::AlgoFactory, Self::Error> {
        let logger_factory = match self.logger_factory {
            Some(value) => value,
            None => build_default_logger_factory(),
        };

        Ok(Self::AlgoFactory::new(logger_factory))
    }
}
