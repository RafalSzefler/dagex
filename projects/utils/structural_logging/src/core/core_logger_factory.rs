use std::sync::Arc;

use immutable_string::ImmutableString;

use crate::traits::StructuralLoggerFactory;

use super::{background_worker::BackgroundWorker, CoreLogger};

pub struct CoreLoggerFactory {
    worker: Arc<BackgroundWorker>,
}

impl CoreLoggerFactory {
    pub(super) fn new(worker: Arc<BackgroundWorker>) -> Self {
        Self { worker }
    }
}

impl StructuralLoggerFactory for CoreLoggerFactory {
    type Logger = CoreLogger;

    fn create(&self, name: &ImmutableString) -> Self::Logger {
        CoreLogger::new(name.clone(), self.worker.clone())
    }
}
