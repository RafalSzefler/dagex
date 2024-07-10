use structural_logging::core::CoreLogger;

use crate::traits::Algorithm;

use super::EpisodeFeasabilityInput;


pub struct EpisodeFeasabilityAlgorithm<'a> {
    input: EpisodeFeasabilityInput<'a>,
    logger: CoreLogger,
}

impl<'a> EpisodeFeasabilityAlgorithm<'a> {
    pub(super) fn new(
        input: EpisodeFeasabilityInput<'a>,
        logger: CoreLogger) -> Self
    {
        Self { input, logger }
    }
}

impl<'a> Algorithm<'a> for EpisodeFeasabilityAlgorithm<'a> {
    type Input<'b> = EpisodeFeasabilityInput<'b>;

    type Output<'b> = ();

    type Error = ();

    fn run(self) -> Result<Self::Output<'a>, Self::Error> {
        todo!()
    }
}
