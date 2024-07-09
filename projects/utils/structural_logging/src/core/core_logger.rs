use std::sync::Arc;

use immutable_string::ImmutableString;

use crate::{models, traits::{StructuralLog, StructuralLogger}};

use super::background_worker::BackgroundWorker;

pub struct CoreLogger {
    name: ImmutableString,
    worker: Arc<BackgroundWorker>,
}

impl CoreLogger {
    pub(super) fn new(name: ImmutableString, worker: Arc<BackgroundWorker>) -> Self {
        Self { name, worker }
    }
}


impl StructuralLogger for CoreLogger {
    fn log<T>(&self, log: T) where T : StructuralLog {
        let mut log_data = log.log_data();
        log_data.update_data(models::keys::logger_name(), self.name.clone());
        self.worker.send_log(log_data);
    }
}
