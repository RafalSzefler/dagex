use std::sync::Arc;

use immutable_string::ImmutableString;

use crate::traits::{LogLevel, StructuralLoggerFactory};

use super::{background_worker::BackgroundWorker, CoreLogger};

pub struct CoreLoggerFactory {
    log_level: LogLevel,
    worker: Arc<BackgroundWorker>,
}

impl CoreLoggerFactory {
    pub(super) fn new(
        log_level: LogLevel,
        worker: Arc<BackgroundWorker>) -> Self
    {
        Self { log_level, worker }
    }
}

impl StructuralLoggerFactory for CoreLoggerFactory {
    type Logger = CoreLogger;

    fn create(&self, name: &ImmutableString) -> Self::Logger {
        CoreLogger::new(
            self.log_level,
            name.clone(),
            self.worker.clone())
    }
}
